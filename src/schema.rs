use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEvent {
    pub msg: String,
}

#[cfg(test)]
impl CustomEvent {
    pub fn mock() -> Self {
        Self {
            msg: "test_msg".to_string(),
        }
    }
}
