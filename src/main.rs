use lambda_func::func;
use lambda_runtime::{Error, service_fn};
use tokio;
use tracing::{self, info};
use tracing_subscriber;

mod errors;
mod lambda_func;
mod schema;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();

    info!("Initializing lambda function...");
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::schema::CustomEvent;

    use super::*;
    use lambda_runtime::{Context, LambdaEvent};

    #[tokio::test]
    async fn test_lambda_func() {
        let event = LambdaEvent::new(CustomEvent::mock(), Context::default());
        let result = func(event).await;

        match result {
            Ok(msg) => assert_eq!(msg, "test_msg".to_string()),
            _ => panic!("expected Ok(msg), not Err"),
        }
    }
}
