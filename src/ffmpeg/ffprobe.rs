use super::schema::FFProbe;
use crate::errors::LambdaError;
use std::path::Path;

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

    let ffprobe: FFProbe = serde_json::from_slice(&result.stdout[..])?;

    Ok(ffprobe)
}
