use crate::errors::LambdaError;
use regex::Regex;

#[derive(Debug)]
pub struct S3Url {
    pub bucket: String,
    pub key: String,
}

impl TryFrom<String> for S3Url {
    type Error = LambdaError;
    fn try_from(url: String) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^s3://(?P<bucket>[^/]+)/(?P<key>.+)$")?;

        match re.captures(&url) {
            Some(caps) => {
                let bucket = caps["bucket"].to_string();
                let key = caps["key"].to_string();
                Ok(Self { bucket, key })
            }
            None => Err(LambdaError::InvalidS3UrlError(url.to_string())),
        }
    }
}

#[allow(dead_code)]
pub trait S3UrlParts {
    fn url(&self) -> String;
    fn bucket(&self) -> String;
    fn key(&self) -> String;
    fn basename(&self) -> String;
}

impl S3UrlParts for S3Url {
    fn url(&self) -> String {
        format!("s3://{}/{}", self.bucket, self.key)
    }

    fn bucket(&self) -> String {
        self.bucket.clone()
    }

    fn key(&self) -> String {
        self.key.clone()
    }

    fn basename(&self) -> String {
        self.key
            .split('/')
            .next_back()
            .unwrap_or(&self.key)
            .to_string()
    }

}
