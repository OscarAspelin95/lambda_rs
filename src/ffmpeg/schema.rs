use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct FFMpegArtifact {
    pub files: Vec<PathBuf>,
}
