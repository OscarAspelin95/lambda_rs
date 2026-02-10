resource "aws_dynamodb_table" "default" {
  name           = "dynamodb-${var.environment}"
  hash_key       = "Uuid"
  billing_mode   = "PROVISIONED"
  read_capacity  = 20
  table_class    = "STANDARD"
  write_capacity = 20

  attribute {
    name = "Uuid"
    type = "S"
  }

  tags = {
    Name        = "dynamodb-table-${var.environment}"
    Environment = var.environment
  }
}
