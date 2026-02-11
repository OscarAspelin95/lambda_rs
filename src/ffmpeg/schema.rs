use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CodecName {
    Png,
    Jpg,
    Gif,
    #[serde(other)]
    Unsupported,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CodecType {
    Video,
    Image,
    #[serde(other)]
    Unsupported,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Streams {
    codec_name: CodecName,
    codec_type: CodecType,
    width: usize,
    height: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Format {
    filename: String,
    size: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FFProbe {
    pub streams: Vec<Streams>,
    pub format: Format,
}
