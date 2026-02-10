use lambda_func::func;
use lambda_runtime::{Error, service_fn};
use tokio;
use tracing::{self, info};
use tracing_subscriber;

mod errors;
mod lambda_func;
mod s3;
mod schema;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().init();

    info!("Initializing lambda function...");
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}
