# aws_lambda_rs
Deploy and run a Rust AWS lambda function.

## Requirements
```
ziglang
rust
cargo-lambda
```

## Building
```bash
cargo lambda build --release --x86-64 -o Zip`
```

## AWS IAM
Create IAM role:
```bash
aws iam create-role \
  --role-name lambda-rs-execution-role \
  --assume-role-policy-document file://trust-policy.json
```

Attach basic Lambda execution policy
```bash
aws iam attach-role-policy \
  --role-name lambda-rs-execution-role \
  --policy-arn arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole
```

Get role ARN:
```bash
ROLE_ARN=$(aws iam get-role --role-name lambda-rs-execution-role --query 'Role.Arn' --output text)
```

## deploy
Make sure session is valid:
```bash
aws login
```

Create the Lambda function:
```bash
aws lambda create-function \
  --role $ROLE_ARN \
  --function-name lambda_rs_function \
  --runtime provided.al2023 \
  --timeout 60 \
  --memory-size 128 \
  --package-type Zip \
  --architectures x86_64 \
  --handler bootstrap \
  --zip-file fileb://target/lambda/lambda_rs/bootstrap.zip
```

update an existing function:
```bash
aws lambda update-function-code \
  --function-name lambda_rs_function \
  --zip-file fileb://target/lambda/lambda_rs/bootstrap.zip
```

## invoking
```bash
aws lambda invoke --function-name "lambda_rs_function" --payload '{"msg": "MyMessage"}' --cli-binary-format raw-in-base64-out out.json
```
