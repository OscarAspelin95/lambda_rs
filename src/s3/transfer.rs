use crate::errors::LambdaError;
use crate::s3::S3UrlParts;
use aws_sdk_dynamodb::Client as DynamoDBClient;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::primitives::ByteStream;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio::fs::create_dir_all;
use tracing::info;

/// Split into separate functions for s3 upload and db insert.
/// We need to solve how we get table_name (terraform)?
pub async fn put_object<T>(
    s3_client: &S3Client,
    dynamodb_client: &DynamoDBClient,
    url_parts: &T,
    // Fix
    local_file: &Path,
) -> Result<(), LambdaError>
where
    T: S3UrlParts,
{
    let body = ByteStream::read_from().path(local_file).build().await?;

    info!("uploading file to s3");
    s3_client
        .put_object()
        .bucket(url_parts.bucket())
        .key(url_parts.key())
        .body(body)
        .send()
        .await
        .map_err(|err| LambdaError::S3UploadError(err.to_string()))?;

    info!("uploading url to dynamodb");
    let url_av = AttributeValue::S(url_parts.url());
    let uuid_av = AttributeValue::S(uuid::Uuid::now_v7().to_string());

    let request = dynamodb_client
        .put_item()
        .table_name("dynamodb-development") // FIX ME
        .item("Uuid", uuid_av)
        .item("Url", url_av);

    let response = request
        .send()
        .await
        .map_err(|e| LambdaError::DynamoDBError(e.to_string()))?;

    info!("{:?}", response);

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
        .map_err(|err| LambdaError::S3GetObjectError(err.to_string()))?;

    let data = response
        .body
        .collect()
        .await
        .map_err(|err| LambdaError::S3GetObjectError(err.to_string()))?;
    let bytes = data.into_bytes();

    let basename = url_parts.basename();
    let local_file = format!("{}/{}", outdir.display(), basename);

    let mut file = File::create(&local_file)?;
    file.write_all(&bytes)?;

    Ok(local_file)
}
