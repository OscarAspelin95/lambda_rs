use crate::errors::LambdaError;
use crate::schema::CustomEvent;
use lambda_runtime::LambdaEvent;
use tracing::info;

pub async fn func(event: LambdaEvent<CustomEvent>) -> Result<String, LambdaError> {
    let (event, _context) = event.into_parts();

    info!("Got event: {:?}", event);

    Ok("success".to_string())
}
