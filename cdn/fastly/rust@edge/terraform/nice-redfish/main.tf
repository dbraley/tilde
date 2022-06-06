resource "fastly_service_compute" "demo" {
  name = "rust-demo"

  domain {
    name    = "safely-nice-redfish.edgecompute.app"
    comment = "demo"
  }

  package {
    filename = "${path.root}/../../pkg/rust-demo.tar.gz"
    source_code_hash = filesha512("${path.root}/../../pkg/rust-demo.tar.gz")
  }

  backend {
    name    = "originless"
    address = "127.0.0.1"
    port    = 80
  }
}