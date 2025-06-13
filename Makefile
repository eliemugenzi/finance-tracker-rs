.PHONY: all run test migrate migrate-add migrate-revert prepare

all: run

run:
	cargo run

test:
	cargo test

migrate:
	dotenvx run -- sqlx migrate run

migrate-add:
	@if [ -z "$(name)" ]; then echo "Error: Migration name required. Use: make migrate-add name=<migration_name>"; exit 1; fi
	dotenvx run -- sqlx migrate add $(name)

migrate-revert:
	dotenvx run -- sqlx migrate revert

prepare:
	dotenvx run -- cargo sqlx prepare