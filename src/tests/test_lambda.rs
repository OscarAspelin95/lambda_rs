use super::utils::{create_dynamodb_table, ensure_bucket, get_object_bytes};
use crate::errors::LambdaError;
use crate::lambda_func::func;
use crate::s3::{S3Url, S3UrlParts, get_clients};
use crate::schema::CustomEvent;

use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::primitives::ByteStream;
use dotenv;
use lambda_runtime::{Context, LambdaEvent};
use uuid::Uuid;

/// setup for making lambda function run with minio:
/// - create input/output bucket for input files.
/// - upload input file to bucket.
async fn get_mock_event(client: &S3Client, msg: &'static [u8]) -> Result<CustomEvent, LambdaError> {
    let input_url = S3Url::try_from("s3://test-input-bucket/test_input.txt".to_string())?;

    ensure_bucket(client, &input_url.bucket).await?;
    ensure_bucket(client, "test-output-bucket").await?;

    let body = ByteStream::from_static(msg);

    client
        .put_object()
        .bucket(&input_url.bucket)
        .key(&input_url.key)
        .body(body)
        .send()
        .await
        .map_err(|e| LambdaError::UnknownError(e.to_string()))?;

    let event = CustomEvent {
        input_s3_url: input_url.url(),
    };

    Ok(event)
}

#[tokio::test]
async fn test_lambda_func() -> Result<(), LambdaError> {
    // read env variables.
    dotenv::dotenv().ok();

    let clients = get_clients().await;
    create_dynamodb_table(&clients.dynamodb).await?;

    let msg: &'static [u8] = b"test-message";
    let mock_event = get_mock_event(&clients.s3, msg).await?;
    let event = LambdaEvent::new(mock_event.clone(), Context::default());
    let result = func(event).await?;

    // Make sure we get back a valid Uuid.
    let uuid = Uuid::parse_str(&result.uuid);
    assert!(uuid.is_ok());

    // Verify the response structure.
    assert!(!result.output_urls.is_empty());
    assert_eq!(result.status, "passthrough");

    // Verify the uploaded file content via the output URL.
    let output_s3_url = S3Url::try_from(result.output_urls[0].clone())?;
    let bytes = get_object_bytes(&clients.s3, &output_s3_url).await?;
    assert_eq!(&bytes[..], msg);

    Ok(())
}
