use crate::{errors::LambdaError, ffprobe::schema::CodecName};
use std::path::{Path, PathBuf};

/// Returns a file path with the file extension swapped.
/// e.g., /path/file.mp4 -> /path/file.gif
pub fn with_new_extension(
    input_file: &Path,
    codec_name: CodecName,
) -> Result<PathBuf, LambdaError> {
    if !input_file.exists() {
        return Err(LambdaError::FileDoesNotExistError(
            input_file.display().to_string(),
        ));
    }

    let base = match input_file
        .file_prefix()
        .map(|pf| format!("{}.{}", pf.display(), codec_name))
    {
        Some(base) => base,
        None => {
            return Err(LambdaError::MissingFileExtensionError(
                input_file.display().to_string(),
            ));
        }
    };

    let outfile = match input_file.parent().map(|p| p.join(base)) {
        Some(outfile) => outfile,
        None => {
            return Err(LambdaError::MissingParentDirectoryError(
                input_file.display().to_string(),
            ));
        }
    };

    Ok(outfile)
}
