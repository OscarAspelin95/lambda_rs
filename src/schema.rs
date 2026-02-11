use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomEvent {
    pub input_s3_url: String,
    pub output_s3_url: String,
}

#[cfg(test)]
impl CustomEvent {
    pub fn mock() -> Self {
        Self {
            input_s3_url: "s3://input-test/test.txt".to_string(),
            output_s3_url: "s3://output-test/success.txt".to_string(),
        }
    }
}
