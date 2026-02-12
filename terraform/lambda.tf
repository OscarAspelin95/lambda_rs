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
  environment {
    variables = {
      "DYNAMODB_TABLE"   : aws_dynamodb_table.default.name,
      "S3_OUTPUT_BUCKET" : aws_s3_bucket.output.id
    }
  }
  layers = [aws_lambda_layer_version.ffmpeg_layer.arn]
}


data "archive_file" "ffmpeg" {
  type        = "zip"
  source_dir  = "../layer/ffmpeg"
  output_path = "../layer/ffmpeg.zip"
}

resource "aws_s3_object" "ffmpeg_layer" {
  bucket = aws_s3_bucket.deployment.id
  key    = "layers/ffmpeg-${filebase64sha256(data.archive_file.ffmpeg.output_path)}.zip"
  source = data.archive_file.ffmpeg.output_path
  etag   = filemd5(data.archive_file.ffmpeg.output_path)
}

resource "aws_lambda_layer_version" "ffmpeg_layer" {
  s3_bucket                = aws_s3_bucket.deployment.id
  s3_key                   = aws_s3_object.ffmpeg_layer.key
  description              = "FFmpeg and ffprobe binaries"
  layer_name               = "ffmpeg-${var.environment}"
  source_code_hash         = data.archive_file.ffmpeg.output_base64sha256
  compatible_architectures = ["x86_64"]
  compatible_runtimes      = ["provided.al2023"]
}
