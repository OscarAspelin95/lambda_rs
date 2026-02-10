use crate::errors::LambdaError;
use crate::s3::S3UrlParts;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::primitives::ByteStream;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio::fs::create_dir_all;

pub async fn put_object<T>(
    client: &S3Client,
    url_parts: &T,
    // Fix
    local_file: &Path,
) -> Result<(), LambdaError>
where
    T: S3UrlParts,
{
    let body = ByteStream::read_from().path(local_file).build().await?;

    client
        .put_object()
        .bucket(url_parts.bucket())
        .key(url_parts.key())
        .body(body)
        .send()
        .await
        .map_err(|err| LambdaError::S3UploadError(err.to_string()))?;

    Ok(())
}

pub async fn get_object<T>(
    client: &S3Client,
    url_parts: &T,
    outdir: &Path,
) -> Result<String, LambdaError>
where
    T: S3UrlParts,
{
    if !outdir.exists() {
        create_dir_all(outdir).await?;
    }

    let response = client
        .get_object()
        .bucket(&url_parts.bucket())
        .key(&url_parts.key())
        .send()
        .await
        .map_err(|err| LambdaError::UnknownError(err.to_string()))?;

    let data = response
        .body
        .collect()
        .await
        .map_err(|err| LambdaError::UnknownError(err.to_string()))?;
    let bytes = data.into_bytes();

    let basename = url_parts.basename();
    let local_file = format!("{}/{}", outdir.display(), basename);

    let mut file = File::create(&local_file).expect("Failed to create file");
    file.write_all(&bytes).expect("Failed to write to file");

    Ok(local_file)
}
