resource "aws_s3_bucket" "default" {
  bucket_prefix = "s3-${var.environment}-"
  force_destroy = true
  region        = var.aws_region

  tags = {
    Name        = "s3_${var.environment}"
    Environment = "${var.environment}"
  }
}
