use aws_sdk_s3::primitives::ByteStreamError;
use lambda_runtime::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LambdaError {
    #[error("Unknown error: {0}")]
    UnknownError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid S3 URL: {0}")]
    InvalidS3UrlError(String),

    #[error("S3 upload error: {0}")]
    S3UploadError(String),

    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("BytestreamError")]
    ByteStreamError(#[from] ByteStreamError),

    #[error("DynamoDBError: {0}")]
    DynamoDBError(String),

    #[error("S3GetObjectError: {0}")]
    S3GetObjectError(String),

    #[error("EnvError: {0}")]
    EnvError(String),

    #[error("Command error: {0}")]
    CommandError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),

    #[error("File does not exist: {0}")]
    FileDoesNotExistError(String),

    #[error("Provided file does not have an extension: `{0}`")]
    MissingFileExtensionError(String),

    #[error("Provided file does not have a parent directory: `{0}`")]
    MissingParentDirectoryError(String),
}

impl From<LambdaError> for Diagnostic {
    fn from(value: LambdaError) -> Diagnostic {
        let (error_type, error_message) = match value {
            LambdaError::UnknownError(error_message) => ("UnknownError", error_message),
            LambdaError::SerializationError(error_message) => ("SerializationError", error_message),
            LambdaError::InvalidS3UrlError(error_message) => ("InvalidS3Url", error_message),
            LambdaError::S3UploadError(error_message) => ("S3UploadError", error_message),
            LambdaError::RegexError(e) => ("RegexError", e.to_string()),
            LambdaError::IoError(e) => ("IoError", e.to_string()),
            LambdaError::ByteStreamError(e) => ("ByteStreamError", e.to_string()),
            LambdaError::DynamoDBError(error_message) => ("DynamoDBError", error_message),
            LambdaError::S3GetObjectError(error_message) => ("S3GetObjectError", error_message),
            LambdaError::EnvError(error_message) => ("EnvError", error_message),
            LambdaError::CommandError(error_message) => ("CommandError", error_message),
            LambdaError::DeserializationError(e) => ("DeserializationError", e.to_string()),
            LambdaError::FileDoesNotExistError(error_message) => {
                ("FileDoesNotExistError", error_message)
            }
            LambdaError::MissingFileExtensionError(error_message) => {
                ("MissingFileExtensionError", error_message)
            }
            LambdaError::MissingParentDirectoryError(error_message) => {
                ("MissingParentDirectoryError", error_message)
            }
        };

        Diagnostic {
            error_type: error_type.into(),
            error_message,
        }
    }
}
