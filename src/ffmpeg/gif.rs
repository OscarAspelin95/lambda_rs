use crate::ffmpeg::FFMpegArtifact;
use crate::ffprobe::schema::CodecName;
use crate::{errors::LambdaError, file_utils::with_new_extension};
use std::path::Path;
use tracing::{debug, error, instrument};

#[instrument(name = "video_to_gif", err)]
pub fn video_to_gif(video: &Path) -> Result<FFMpegArtifact, LambdaError> {
    let gif = with_new_extension(video, CodecName::Gif)?;

    let mut cmd = std::process::Command::new("ffmpeg");

    cmd.arg("-i")
        .arg(video.display().to_string())
        .args(["-ss", "00:00:00"])
        .args(["-t", "5"])
        .args(["-vf", "fps=10,scale=480:-1:flags=lanczos"])
        .args(["-loop", "0"])
        .arg(gif.display().to_string());

    let output = cmd
        .output()
        .map_err(|e| LambdaError::CommandError(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!(exit_code = ?output.status.code(), %stderr, "ffmpeg video_to_gif failed");
    }

    if !gif.is_file() {
        return Err(LambdaError::FileDoesNotExistError(
            gif.display().to_string(),
        ));
    }

    debug!(output = %gif.display(), "GIF created");

    Ok(FFMpegArtifact { files: vec![gif] })
}
