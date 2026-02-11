# aws_lambda_rs
Let's build, deploy and run a basic Rust AWS lambda function.

This particular function only downloads a file from s3, uploads it to another bucket and writes the output url to dynamodb. However, with these concepts in place, it can easily be extended to perform more powerful tasks.


## Requirements

- AWS account (with credits)
- AWS cli
- ziglang
- rust
- cargo-lambda
- terraform
- docker


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
