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

resource "aws_iam_role" "lambda_execution_role" {
  name               = "lambda_execution_role_${var.environment}"
  assume_role_policy = data.aws_iam_policy_document.assume_role.json
}


resource "aws_iam_policy" "s3_input_read_only" {
  name        = "s3-input-read-only-${var.environment}"
  description = "Allow read-only access to s3 input bucket"
  policy = jsonencode(
    {
      Version = "2012-10-17"
      Statement = [
        {
          Effect = "Allow"
          Action = [
            "s3:GetObject"
          ]
          Resource = "arn:aws:s3:::${aws_s3_bucket.input.bucket}/*"
        },
        {
          Effect = "Allow"
          Action = [
            "s3:ListBucket"
          ]
          Resource = "arn:aws:s3:::${aws_s3_bucket.input.bucket}"
        },
      ]
    }
  )
}

resource "aws_iam_policy" "s3_output_put_only" {
  name        = "s3-output-put-only-${var.environment}"
  description = "Allow put access to s3 output bucket"
  policy = jsonencode(
    {
      Version = "2012-10-17"
      Statement = [
        {
          Effect = "Allow"
          Action = [
            "s3:PutObject"
          ]
          Resource = "arn:aws:s3:::${aws_s3_bucket.output.bucket}/*"
        },
      ]
    }
  )
}


resource "aws_iam_policy" "dynamodb_table_write" {
  name        = "dynamodb-table-write-${var.environment}"
  description = "Allow write access to dynamodb table"
  policy = jsonencode(
    {
      Version = "2012-10-17"
      Statement = [
        {
          Effect = "Allow"
          Action = [
            "dynamodb:PutItem",
          ]
          Resource = aws_dynamodb_table.default.arn
        },
      ]
    }
  )
}

resource "aws_iam_role_policy_attachment" "lambda_s3_readonly" {
  role       = aws_iam_role.lambda_execution_role.name
  policy_arn = aws_iam_policy.s3_input_read_only.arn
}

resource "aws_iam_role_policy_attachment" "lambda_s3_putonly" {
  role       = aws_iam_role.lambda_execution_role.name
  policy_arn = aws_iam_policy.s3_output_put_only.arn
}

resource "aws_iam_role_policy_attachment" "lambda_logs" {
  role       = aws_iam_role.lambda_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_iam_role_policy_attachment" "lambda_dynamodb_write" {
  role       = aws_iam_role.lambda_execution_role.name
  policy_arn = aws_iam_policy.dynamodb_table_write.arn
}
