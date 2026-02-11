use crate::errors::LambdaError;
use crate::ffmpeg::run_ffprobe;
use crate::s3::{S3Url, get_clients, get_object, put_object};
use crate::schema::CustomEvent;
use lambda_runtime::LambdaEvent;
use tempfile::TempDir;
use tracing::{info, instrument};

#[instrument(name = "lambda-handler-function")]
pub async fn func(event: LambdaEvent<CustomEvent>) -> Result<String, LambdaError> {
    info!("Getting S3 client...");
    let clients = get_clients().await;

    info!("Got event: {:?}", event);

    // Download file.
    let s3_url_input = S3Url::try_from(event.payload.input_s3_url)?;
    let outdir = TempDir::new()?;
    let local_file = get_object(&clients.s3, &s3_url_input, outdir.path()).await?;

    // Run ffprobe
    let ffprobe = run_ffprobe(&local_file).ok();

    // Upload file.
    let s3_url_output = S3Url::try_from(event.payload.output_s3_url)?;
    let id = put_object(
        &clients.s3,
        &clients.dynamodb,
        &s3_url_output,
        ffprobe,
        &local_file,
    )
    .await?;

    Ok(id.to_string())
}
