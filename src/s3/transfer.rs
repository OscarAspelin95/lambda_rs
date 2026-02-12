use crate::errors::LambdaError;
use crate::s3::S3UrlParts;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::primitives::ByteStream;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use tokio::fs::create_dir_all;
use tokio::io::AsyncWriteExt;
use tracing::{debug, info, instrument};

#[instrument(name = "upload_to_s3", err)]
pub async fn upload_to_s3(
    s3_client: &S3Client,
    bucket: &str,
    key: &str,
    local_file: &Path,
) -> Result<String, LambdaError> {
    let body = ByteStream::read_from().path(local_file).build().await?;

    info!("uploading to s3://{}/{}", bucket, key);
    s3_client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await
        .map_err(|err| LambdaError::S3UploadError(err.to_string()))?;

    let url = format!("s3://{}/{}", bucket, key);
    Ok(url)
}

#[instrument(name = "get_object", err)]
pub async fn get_object<T>(
    client: &S3Client,
    url_parts: &T,
    outdir: &Path,
) -> Result<PathBuf, LambdaError>
where
    T: S3UrlParts + Debug,
{
    if !outdir.exists() {
        create_dir_all(outdir).await?;
    }

    info!("downloading s3://{}/{}", url_parts.bucket(), url_parts.key());

    let response = client
        .get_object()
        .bucket(url_parts.bucket())
        .key(url_parts.key())
        .send()
        .await
        .map_err(|err| LambdaError::S3GetObjectError(err.to_string()))?;

    let content_length = response.content_length().unwrap_or(0);
    debug!(content_length, "starting stream to disk");

    let basename = url_parts.basename();
    let local_file = format!("{}/{}", outdir.display(), basename);

    let mut file = tokio::fs::File::create(&local_file).await?;
    let mut body = response.body;
    let mut bytes_written: u64 = 0;

    while let Some(chunk) = body
        .try_next()
        .await
        .map_err(|err| LambdaError::S3GetObjectError(err.to_string()))?
    {
        bytes_written += chunk.len() as u64;
        file.write_all(&chunk).await?;
    }
    file.flush().await?;

    debug!(bytes_written, file = %local_file, "download complete");

    Ok(PathBuf::from(local_file))
}
