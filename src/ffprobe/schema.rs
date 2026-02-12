use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum CodecName {
    // Image
    Png,
    Jpg,
    Svg,
    // Video
    Gif,
    Mp4,
    Mov,
    Avi,
    #[serde(other)]
    Unknown,
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
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_streams")]
    pub streams: Vec<Streams>,
    pub format: Format,
}

/// This is not ideal, but enables us to only deserialize streams that
/// follows our defined `Streams` struct.
fn deserialize_streams<'de, D>(deserializer: D) -> Result<Vec<Streams>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let values: Vec<serde_json::Value> = Vec::deserialize(deserializer)?;
    Ok(values
        .into_iter()
        .filter_map(|v| serde_json::from_value(v).ok())
        .collect())
}

impl FFProbe {
    pub fn is_video(&self) -> bool {
        self.streams
            .iter()
            .any(|stream| matches!(stream.codec_type, CodecType::Video))
    }

    pub fn is_image(&self) -> bool {
        self.streams
            .iter()
            .any(|stream| matches!(stream.codec_type, CodecType::Image))
    }

    pub fn is_png(&self) -> bool {
        self.streams
            .iter()
            .any(|stream| matches!(stream.codec_name, CodecName::Png))
    }
}
