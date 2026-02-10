use crate::s3::{S3Url, S3UrlParts};
use rstest::*;

#[rstest]
#[case("s3://bucket/my_key".into(), "bucket".into(), "my_key".into(), "my_key".into(), None)]
#[case("s3://bucket/my/key/basename.txt".into(), "bucket".into(), "my/key/basename.txt".into(), "basename.txt".into(), Some("txt".into()))]
#[case("s3://bucket/my/key/basename.tar.gz".into(), "bucket".into(), "my/key/basename.tar.gz".into(), "basename.tar.gz".into(), Some("tar.gz".into()))]
fn test_s3_url(
    #[case] url: String,
    #[case] expected_bucket: String,
    #[case] expected_key: String,
    #[case] expected_basename: String,
    #[case] expected_ext: Option<String>,
) {
    let s3_url = S3Url::try_from(url).expect("Invalid url");

    assert_eq!(s3_url.bucket, expected_bucket);
    assert_eq!(s3_url.key, expected_key);
    assert_eq!(s3_url.basename(), expected_basename);
    assert!(s3_url.key.ends_with(&s3_url.basename()))
}
