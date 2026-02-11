# aws_lambda_rs
Let's build, deploy and run a basic Rust AWS lambda function.

This a work in progress, but the goal is to have a AWS lambda function that:
- Accepts different kinds of media files (video, image, etc) stored in s3.
- Processes these with `ffmpeg`.
- Uploads the processed artifacts on s3 


## Requirements

- AWS account (with credits)
- AWS cli
- ziglang
- rust
- cargo-lambda
- terraform
- docker
- ffmpeg and ffprobe binaries


# FFMPEG binaries
Our lambda function will have a layer containing the `ffmpeg` and `ffprobe` binaries. These must be present in the root directory as
```bash
./layer/ffmpeg/bin/ffmpeg
./layer/ffmpeg/bin/ffprobe
```

Typically, you'd do something like this (considering that the version might be different).
```bash
mkdir -p ./layer/ffmpeg/bin && cd ./layer/ffmpeg/bin
wget https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz
tar -xvf ffmpeg-release-amd64-static.tar.xz
mv ffmpeg-7.0.2-amd64-static/ffmpeg ffmpeg-7.0.2-amd64-static/ffprobe .

# cleanup
rm ffmpeg-release-amd64-static.tar.xz
rm -rf ffmpeg-7.0.2-amd64-static/ffmpeg
```

# Running with terraform
Configure
```bash
cd terraform
terraform init
terraform validate
```

Build binary (no zip, terraform handles this).
```bash
cargo lambda build --release --x86-64
```

Deploy with terraform
```bash
cd terraform
terraform plan
terraform apply
```

Get resource info such as function name, buckets, etc.
```bash
# all resources
terraform output -json > resources.json

# specific key
terraform output -json | jq .<key>.value
```

Invoke function
```bash
aws lambda invoke --function-name <function_name> --payload file://payload.json --cli-binary-format raw-in-base64-out out.json
```

with a `payload.json` file that specifies the input and output s3 urls. Check the terraform output to get the values for `<input_bucket>` and `<output_bucket>`. Remember that `s3://<input_bucket>/<file>` must exist. 
```json
{
	"input_s3_url": "s3://<input_bucket>/<file>",
	"output_s3_url": "s3://<output_bucket>/<file>"
}
```

# Local Development
Uses `docker-compose` with the following service:
- minio (mocks aws s3)
- dynamodb-local (mocks aws dynamodb)
- dynamodb-ui

## Environment
Requires a `.env` file:
```
AWS_ENDPOINT_URL_S3=http://localhost:9000
AWS_ENDPOINT_URL_DYNAMODB=http://localhost:8000
AWS_REGION=""
AWS_ACCESS_KEY_ID=""
AWS_SECRET_ACCESS_KEY=""
DYNAMODB_TABLE=""

## docker minio
MINIO_ROOT_USER=""
MINIO_ROOT_PASSWORD=""

## docker dynamodb-ui
DYNAMO_ENDPOINT=http://dynamodb:8000
```

## Getting started
Start services
```bash
make docker-start
```

Test functionality
```bash
cargo test
```

UI Endpoints

| Service | Url |
| --- | ----------- |
| MinIO UI | http://localhost:9001 |
| DynamoDB UI | http://localhost:8001 |
