use lambda_func::func;
use lambda_runtime::{Error, service_fn};
use tracing::info;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;
mod errors;
mod ffmpeg;
mod lambda_func;
mod s3;
mod schema;

#[cfg(test)]
mod tests;

fn setup_tracing() {
    let formatting_layer = BunyanFormattingLayer::new("axum_tracing".into(), std::io::stdout);
    let subscriber = Registry::default()
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
