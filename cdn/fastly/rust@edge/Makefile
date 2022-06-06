HOST:=127.0.0.1
PORT:=7676

serve run:
	fastly compute serve --watch --addr="$(HOST):$(PORT)"

validate check:
	terraform -chdir=terraform/nice-redfish/ validate

plan:
	terraform -chdir=terraform/nice-redfish/ plan

deploy apply:
	terraform -chdir=terraform/nice-redfish/ apply

tail log-tail:
	fastly log-tail

fix:
	cargo fmt