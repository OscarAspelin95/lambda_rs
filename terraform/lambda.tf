// allows lambda to assume a role.
data "aws_iam_policy_document" "assume_role" {
  version = "2012-10-17"
  statement {
    effect = "Allow"
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
    actions = ["sts:AssumeRole"]

  }
}

// define the role and attach policy document.
resource "aws_iam_role" "lambda_execution_role" {
  name               = "lambda_execution_role"
  assume_role_policy = data.aws_iam_policy_document.assume_role.json
}


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
