provider "aws" {
  region  = "eu-central-1"
  profile = "julien"
}

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 3.27"
    }
  }

  required_version = ">= 0.14.9"
}

resource "aws_iam_role" "iam_for_lambda" {
  name = "iam_for_lambda"
  assume_role_policy = data.aws_iam_policy_document.lambdas.json
}

data "aws_iam_policy_document" "lambdas" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
    effect = "Allow"
  }
}

data "archive_file" "zip" {
  type        = "zip"
  source_file = "../first/hello/main" # TODO: Move to variable
  output_path = "../first/hello/main.zip" # TODO: Move to variable
}

resource "aws_lambda_function" "test_lambda" {
  filename      = data.archive_file.zip.output_path
  function_name = "lambda_function_name" # TODO: MOve to variable
  role          = aws_iam_role.iam_for_lambda.arn
  handler       = "main"

  source_code_hash = filebase64sha256(data.archive_file.zip.output_path)

  runtime = "go1.x"

  environment {
    variables = {
      foo = "bar"
    }
  }
}