use lambda_func::func;
use lambda_runtime::{Error, service_fn};
use tracing::info;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter};
mod dispatch;
mod dynamodb;
mod errors;
mod ffmpeg;
mod ffprobe;
mod file_utils;
mod lambda_func;
mod s3;
mod schema;

#[cfg(test)]
mod tests;

fn setup_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("lambda-rs".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    info!("Initializing lambda function...");
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}
