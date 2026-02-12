use std::path::Path;

use crate::ffmpeg::{FFMpegArtifact, video_to_gif};
use crate::ffprobe::schema::FFProbe;

pub fn ffmpeg_dispatch(file: &Path, ffprobe: &FFProbe) -> Option<FFMpegArtifact> {
    if ffprobe.is_video() {
        Some(video_to_gif(file));
    }

    None
}
