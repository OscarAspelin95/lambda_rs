resource "aws_s3_bucket" "input" {
  bucket_prefix = "s3-input-${var.environment}-"
  force_destroy = true
  region        = var.aws_region

  tags = {
    Name        = "s3_input_${var.environment}"
    Environment = "${var.environment}"
  }
}


resource "aws_s3_bucket" "output" {
  bucket_prefix = "s3-output-${var.environment}-"
  force_destroy = true
  region        = var.aws_region

  tags = {
    Name        = "s3_output_${var.environment}"
    Environment = "${var.environment}"
  }
}


resource "aws_s3_bucket" "deployment" {
  bucket_prefix = "deployment-artifacts-${var.environment}-"
  force_destroy = true
  region        = var.aws_region

  tags = {
    Name        = "deployment_artifacts_${var.environment}"
    Environment = "${var.environment}"
  }
}
