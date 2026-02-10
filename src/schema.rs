use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEvent {
    pub input_s3_url: String,
    pub output_s3_url: String,
}

#[cfg(test)]
impl CustomEvent {
    pub fn mock() -> Self {
        Self {
            input_s3_url: "s3://input_bucket/key".to_string(),
            output_s3_url: "s3://output_bucket/key".to_string(),
        }
    }
}
