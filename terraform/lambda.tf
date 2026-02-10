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


// policy for s3 bucket readonly
resource "aws_iam_policy" "lambda_s3_read_only" {
  name        = "lambda-s3-read-only"
  description = "Allow read-only access to s3 bucket"
  policy = jsonencode(
    {
      Version = "2012-10-17"
      Statement = [
        {
          Effect = "Allow"
          Action = [
            "s3:GetObject"
          ]
          Resource = "arn:aws:s3:::${aws_s3_bucket.default.bucket}/*"
        },
        {
          Effect = "Allow"
          Action = [
            "s3:ListBucket"
          ]
          Resource = "arn:aws:s3:::${aws_s3_bucket.default.bucket}"
        },
      ]
    }
  )
}

// attach policy to role
resource "aws_iam_role_policy_attachment" "lambda_s3_readonly" {
  role       = aws_iam_role.lambda_execution_role.name
  policy_arn = aws_iam_policy.lambda_s3_read_only.arn
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
