variable "aws_region" {
  type    = string
  default = "eu-north-1"

  validation {
    condition     = contains(["eu-north-1"], var.aws_region)
    error_message = "Currently, can only use eu-north-1"
  }
}


variable "environment" {
  type    = string
  default = "development"

  validation {
    condition     = contains(["development", "staging", "production"], var.environment)
    error_message = "Must be `development`, `staging`, or `production`"
  }
}
