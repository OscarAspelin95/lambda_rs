use crate::errors::LambdaError;
use crate::ffprobe::schema::FFProbe;
use aws_sdk_dynamodb::Client as DynamoDBClient;
use aws_sdk_dynamodb::types::AttributeValue;
use tracing::{debug, instrument};
use uuid::Uuid;

#[instrument(name = "write_to_dynamodb", err)]
pub async fn write_to_dynamodb(
    dynamodb_client: &DynamoDBClient,
    uuid: &Uuid,
    output_urls: &[String],
    ffprobe: Option<&FFProbe>,
) -> Result<(), LambdaError> {
    let table_name =
        std::env::var("DYNAMODB_TABLE").map_err(|e| LambdaError::EnvError(e.to_string()))?;

    let uuid_av = AttributeValue::S(uuid.to_string());
    let urls_av = AttributeValue::L(
        output_urls
            .iter()
            .map(|u| AttributeValue::S(u.clone()))
            .collect(),
    );
    let ffprobe_av = AttributeValue::S(serde_json::to_string(&ffprobe)?);

    let request = dynamodb_client
        .put_item()
        .table_name(&table_name)
        .item("Uuid", uuid_av)
        .item("Urls", urls_av)
        .item("Meta", ffprobe_av);

    request
        .send()
        .await
        .map_err(|e| LambdaError::DynamoDBError(e.to_string()))?;

    debug!(%uuid, table = %table_name, "wrote metadata to dynamodb");

    Ok(())
}
