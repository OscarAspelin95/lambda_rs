use crate::dispatch::ffmpeg_dispatch;
use crate::dynamodb::write_to_dynamodb;
use crate::errors::LambdaError;
use crate::ffprobe::run_ffprobe;
use crate::s3::{S3Url, get_clients, get_object, upload_to_s3};
use crate::schema::{CustomEvent, LambdaResponse};
use lambda_runtime::LambdaEvent;
use tempfile::TempDir;
use tracing::{debug, info, instrument, warn};
use uuid::Uuid;

#[instrument(name = "lambda-handler-function", err)]
pub async fn func(event: LambdaEvent<CustomEvent>) -> Result<LambdaResponse, LambdaError> {
    let clients = get_clients().await;

    let output_bucket = std::env::var("S3_OUTPUT_BUCKET")
        .map_err(|e| LambdaError::EnvError(e.to_string()))?;

    info!("processing event: {:?}", event.payload);

    // Download file.
    let s3_url_input = S3Url::try_from(event.payload.input_s3_url)?;
    let outdir = TempDir::new()?;
    let local_file = get_object(&clients.s3, &s3_url_input, outdir.path()).await?;
    debug!(file = %local_file.display(), "downloaded input file");

    // Run ffprobe.
    let ffprobe = match run_ffprobe(&local_file) {
        Ok(probe) => Some(probe),
        Err(e) => {
            warn!(error = %e, "ffprobe failed, proceeding without media info");
            None
        }
    };

    // Run dispatch if possible.
    let (artifact, media_type, status) = match &ffprobe {
        Some(probe) => {
            let mt = if probe.is_video() {
                "video"
            } else if probe.is_image() {
                "image"
            } else {
                "unknown"
            };
            match ffmpeg_dispatch(&local_file, probe)? {
                Some(art) => {
                    debug!(media_type = mt, files = ?art.files, "dispatch produced artifact");
                    (Some(art), mt.to_string(), "processed".to_string())
                }
                None => {
                    debug!(media_type = mt, "no conversion available, passing through");
                    (None, mt.to_string(), "passthrough".to_string())
                }
            }
        }
        None => (None, "unknown".to_string(), "passthrough".to_string()),
    };

    // Generate UUID for output path.
    let uuid = Uuid::now_v7();
    debug!(%uuid, "generated output uuid");

    // Upload artifacts to S3.
    let mut output_urls = Vec::new();

    if let Some(ref art) = artifact {
        for file in &art.files {
            let filename = file
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| "output".to_string());
            let key = format!("{}/{}", uuid, filename);
            let url = upload_to_s3(&clients.s3, &output_bucket, &key, file).await?;
            output_urls.push(url);
        }
    } else {
        // No artifact — upload the original file as passthrough.
        let filename = local_file
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_else(|| "output".to_string());
        let key = format!("{}/{}", uuid, filename);
        let url = upload_to_s3(&clients.s3, &output_bucket, &key, &local_file).await?;
        output_urls.push(url);
    }

    // Write metadata to DynamoDB.
    write_to_dynamodb(
        &clients.dynamodb,
        &uuid,
        &output_urls,
        ffprobe.as_ref(),
    )
    .await?;

    info!(%uuid, media_type, status, urls = ?output_urls, "request completed");

    Ok(LambdaResponse {
        uuid: uuid.to_string(),
        output_urls,
        media_type,
        status,
    })
}
