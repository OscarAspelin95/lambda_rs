use crate::ffmpeg::FFMpegArtifact;
use crate::ffprobe::schema::CodecName;
use crate::{errors::LambdaError, file_utils::with_new_extension};
use std::path::Path;
use tracing::{debug, error, instrument};

#[instrument(name = "image_to_png", err)]
pub fn image_to_png(image: &Path) -> Result<FFMpegArtifact, LambdaError> {
    let png = with_new_extension(image, CodecName::Png)?;

    let mut cmd = std::process::Command::new("ffmpeg");

    cmd.arg("-i")
        .arg(image.display().to_string())
        .arg(png.display().to_string());

    let output = cmd
        .output()
        .map_err(|e| LambdaError::CommandError(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!(exit_code = ?output.status.code(), %stderr, "ffmpeg image_to_png failed");
    }

    if !png.is_file() {
        return Err(LambdaError::FileDoesNotExistError(
            png.display().to_string(),
        ));
    }

    debug!(output = %png.display(), "PNG created");

    Ok(FFMpegArtifact { files: vec![png] })
}
