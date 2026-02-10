data "archive_file" "default" {
  type        = "zip"
  source_file = "../target/lambda/lambda_rs/bootstrap"
  output_path = "../target/lambda/lambda_rs/bootstrap.zip"
}

resource "aws_lambda_function" "default" {
  function_name = "lambda-rs-${var.environment}"
  region        = var.aws_region
  role          = aws_iam_role.lambda_execution_role.arn
  code_sha256   = data.archive_file.default.output_base64sha256
  runtime       = "provided.al2023"
  architectures = ["x86_64"]
  handler       = "bootstrap"
  timeout       = 60
  package_type  = "Zip"
  filename      = data.archive_file.default.output_path
}
