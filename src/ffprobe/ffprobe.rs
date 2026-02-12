use tracing::{debug, error, instrument};

use super::schema::FFProbe;
use crate::errors::LambdaError;
use std::path::Path;

#[instrument(name = "run_ffprobe", err)]
pub fn run_ffprobe(file: &Path) -> Result<FFProbe, LambdaError> {
    if !file.exists() {
        return Err(LambdaError::FileDoesNotExistError(
            file.display().to_string(),
        ));
    }

    let mut cmd = std::process::Command::new("ffprobe");

    cmd.arg("-v")
        .arg("quiet")
        .arg("-output_format")
        .arg("json")
        .arg("-show_format")
        .arg("-show_streams")
        .arg(file.display().to_string());

    let result = cmd
        .output()
        .map_err(|e| LambdaError::CommandError(e.to_string()))?;

    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        error!(exit_code = ?result.status.code(), %stderr, "ffprobe exited with error");
    }

    let ffprobe: FFProbe = serde_json::from_slice(&result.stdout[..])?;

    debug!(streams = ffprobe.streams.len(), "ffprobe analysis complete");

    Ok(ffprobe)
}
