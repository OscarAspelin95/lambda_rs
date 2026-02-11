output "region" {
  value = var.aws_region
}

output "environment" {
  value = var.environment
}

output "dynamodb_table" {
  value = aws_dynamodb_table.default.name
}

output "input_bucket" {
  value = aws_s3_bucket.input.bucket
}

output "output_bucket" {
  value = aws_s3_bucket.output.bucket
}

output "lambda_function_name" {
  value = aws_lambda_function.default.function_name
}
