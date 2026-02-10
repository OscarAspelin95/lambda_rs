use crate::errors::LambdaError;
use aws_sdk_s3::Client as S3Client;

pub async fn get_client() -> Result<S3Client, LambdaError> {
    let config = aws_config::load_from_env().await;
    let client = S3Client::new(&config);

    Ok(client)
}
