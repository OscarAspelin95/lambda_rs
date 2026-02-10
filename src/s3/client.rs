use aws_sdk_dynamodb::Client as DynamoDBClient;
use aws_sdk_s3::Client as S3Client;

pub struct Clients {
    pub s3: S3Client,
    pub dynamodb: DynamoDBClient,
}

pub async fn get_clients() -> Clients {
    let config = aws_config::load_from_env().await;

    let s3_config = aws_sdk_s3::config::Builder::from(&config)
        .force_path_style(true) // minio requires this, aws s3 tolerates this.
        .build();

    let s3_client = S3Client::from_conf(s3_config);
    let dynamodb_client = DynamoDBClient::new(&config);

    Clients {
        s3: s3_client,
        dynamodb: dynamodb_client,
    }
}
