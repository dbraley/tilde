# Terraform 0.13+ requires providers to be declared in a "required_providers" block
terraform {
  required_providers {
    fastly = {
      source  = "fastly/fastly"
      version = ">= 2.0.0"
    }
  }
}

# Configure the Fastly Provider
provider "fastly" {
  api_key = var.fastly_api_key
}