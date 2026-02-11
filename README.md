# aws_lambda_rs
Deploy and run a Rust AWS lambda function.

## Current status
Currently, the function is basically a very fancy copy-paste.

## Requirements
```
ziglang
rust
cargo-lambda
terraform
docker
```

# Running with terraform
Configure
```bash
cd terraform
terraform init
terraform validate
```

Build binary (no zip).
```bash
cargo lambda build --release --x86-64
```

Deploy with terraform
```bash
cd terraform
terraform plan
terraform apply
```

Invoke function
```bash
aws lambda invoke --function-name "lambda-rs-development" --payload file://payload.json --cli-binary-format raw-in-base64-out out.json
```

with a `payload.json` file like
```json
{
	"input_s3_url": "s3://...",
	"output_s3_url": "s3://..."
}
```

# Local Development
Requires a .env file
```
# rust aws-sdk-s3
AWS_ENDPOINT_URL_S3=http://localhost:9000
AWS_ENDPOINT_URL_DYNAMODB=http://localhost:8000
AWS_REGION=""
AWS_ACCESS_KEY_ID=""
AWS_SECRET_ACCESS_KEY=""
MINIO_ROOT_USER=""
MINIO_ROOT_PASSWORD=""
DYNAMODB_TABLE=""

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

Endpoints
```
MinIO UI 		http://localhost:9001
DynamoDB UI 		http://localhost:8001
```
