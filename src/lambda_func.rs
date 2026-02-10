use std::path::PathBuf;

use crate::errors::LambdaError;
use crate::s3::{S3Url, get_client, get_object, put_object};
use crate::schema::CustomEvent;
use lambda_runtime::LambdaEvent;
use tempfile::TempDir;
use tracing::{info, instrument};

#[instrument(name = "lambda-handler-function")]
pub async fn func(event: LambdaEvent<CustomEvent>) -> Result<String, LambdaError> {
    info!("Getting S3 client...");
    let client = get_client().await?;

    info!("Got event: {:?}", event);

    // Download file.
    let s3_url_input = S3Url::try_from(event.payload.input_s3_url)?;
    let outdir = TempDir::new()?;
    let local_file = get_object(&client, &s3_url_input, outdir.path()).await?;

    // Upload file.
    let pb = PathBuf::from(local_file);
    let s3_url_output = S3Url::try_from(event.payload.output_s3_url)?;
    put_object(&client, &s3_url_output, &pb).await?;

    Ok("success".to_string())
}
