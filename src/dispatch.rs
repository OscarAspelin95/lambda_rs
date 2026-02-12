use std::path::Path;

use crate::errors::LambdaError;
use crate::ffmpeg::{FFMpegArtifact, image_to_png, video_to_gif};
use crate::ffprobe::schema::FFProbe;
use tracing::{debug, instrument};

#[instrument(name = "ffmpeg_dispatch", err, skip(ffprobe))]
pub fn ffmpeg_dispatch(
    file: &Path,
    ffprobe: &FFProbe,
) -> Result<Option<FFMpegArtifact>, LambdaError> {
    if ffprobe.is_video() {
        debug!("dispatching video to GIF conversion");
        return Ok(Some(video_to_gif(file)?));
    }

    if ffprobe.is_image() {
        if ffprobe.is_png() {
            debug!("image is already PNG, passing through");
            return Ok(Some(FFMpegArtifact {
                files: vec![file.to_path_buf()],
            }));
        }
        debug!("dispatching image to PNG conversion");
        return Ok(Some(image_to_png(file)?));
    }

    debug!("unsupported media type, no conversion");
    Ok(None)
}
