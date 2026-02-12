use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomEvent {
    pub input_s3_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LambdaResponse {
    pub uuid: String,
    pub output_urls: Vec<String>,
    pub media_type: String,
    pub status: String,
}
