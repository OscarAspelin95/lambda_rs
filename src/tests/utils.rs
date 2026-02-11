use crate::errors::LambdaError;
use crate::s3::{S3Url, S3UrlParts};
use aws_sdk_dynamodb::types::{AttributeDefinition, BillingMode, KeySchemaElement};
use aws_sdk_dynamodb::{Client as DynamoDBClient, types::KeyType};
use aws_sdk_s3::Client as S3Client;
use bytes::Bytes;

pub async fn ensure_bucket(client: &S3Client, bucket_name: &str) -> Result<(), LambdaError> {
    let response = client.create_bucket().bucket(bucket_name).send().await;

    match response {
        Ok(_) => Ok(()),
        Err(e) => {
            let service_err = e.into_service_error();

            match service_err.is_bucket_already_exists()
                | service_err.is_bucket_already_owned_by_you()
            {
                true => return Ok(()),
                false => return Err(LambdaError::UnknownError(service_err.to_string())),
            };
        }
    }
}

pub async fn create_dynamodb_table(client: &DynamoDBClient) -> Result<(), LambdaError> {
    let table_name = std::env::var("DYNAMODB_TABLE").expect("missing DYNAMODB_TABLE env var");

    let key_schema = KeySchemaElement::builder()
        .key_type(KeyType::Hash)
        .attribute_name("Uuid")
        .build()
        .map_err(|e| LambdaError::DynamoDBError(e.to_string()))?;

    let attribute_schema = AttributeDefinition::builder()
        .attribute_name("Uuid")
        .attribute_type(aws_sdk_dynamodb::types::ScalarAttributeType::S)
        .build()
        .map_err(|e| LambdaError::DynamoDBError(e.to_string()))?;

    let response = client
        .create_table()
        .table_name(table_name)
        .key_schema(key_schema)
        .attribute_definitions(attribute_schema)
        .billing_mode(BillingMode::PayPerRequest)
        .send()
        .await;

    match response {
        Ok(_) => return Ok(()),
        Err(e) => {
            let service_err = e.into_service_error();

            match service_err.is_resource_in_use_exception() {
                true => return Ok(()),
                false => Err(LambdaError::DynamoDBError(service_err.to_string())),
            }
        }
    }
}

pub async fn get_object_bytes<T>(client: &S3Client, url_parts: &T) -> Result<Bytes, LambdaError>
where
    T: S3UrlParts,
{
    let response = client
        .get_object()
        .bucket(&url_parts.bucket())
        .key(&url_parts.key())
        .send()
        .await
        .map_err(|e| LambdaError::S3GetObjectError(e.to_string()))?;

    let bytes = response.body.collect().await?.into_bytes();

    Ok(bytes)
}
