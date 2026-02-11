use aws_sdk_s3::primitives::ByteStreamError;
use lambda_runtime::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LambdaError {
    #[error("Unknown error: {0}")]
    UnknownError(String),

    #[error("Failed to run fastq stats: {0}")]
    FastqStatsError(String),

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
}

impl From<LambdaError> for Diagnostic {
    fn from(value: LambdaError) -> Diagnostic {
        let (error_type, error_message) = match value {
            LambdaError::UnknownError(error_message) => ("UnknownError", error_message),
            LambdaError::FastqStatsError(error_message) => ("FastqStatsError", error_message),
            LambdaError::SerializationError(error_message) => ("SerializationError", error_message),
            LambdaError::InvalidS3UrlError(error_message) => ("InvalidS3Url", error_message),
            LambdaError::S3UploadError(error_message) => ("S3UploadError", error_message),
            LambdaError::RegexError(e) => ("RegexError", e.to_string()),
            LambdaError::IoError(e) => ("IoError", e.to_string()),
            LambdaError::ByteStreamError(e) => ("ByteStreamError", e.to_string()),
            LambdaError::DynamoDBError(error_message) => ("DynamoDBError", error_message),
            LambdaError::S3GetObjectError(error_message) => ("S3GetObjectError", error_message),
            LambdaError::EnvError(error_message) => ("EnvError", error_message),
        };

        Diagnostic {
            error_type: error_type.into(),
            error_message,
        }
    }
}
