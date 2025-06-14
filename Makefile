.PHONY: build run test clean migrate-up migrate-down migrate migrate-add migrate-revert prepare check lint

# Build the project
build:
	cargo build

# Run the project
run:
	cargo run

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Run database migrations up (with dotenvx)
migrate:
	dotenvx run -- sqlx migrate run

# Add a new migration (with dotenvx)
migrate-add:
	@if [ -z "$(name)" ]; then echo "Error: Migration name required. Use: make migrate-add name=<migration_name>"; exit 1; fi
	dotenvx run -- sqlx migrate add $(name)

# Revert the last migration (with dotenvx)
migrate-revert:
	dotenvx run -- sqlx migrate revert

# Prepare SQLx offline mode (with dotenvx)
prepare:
	dotenvx run -- cargo sqlx prepare

# Run database migrations up (direct)
migrate-up:
	sqlx migrate run

# Revert the last migration (direct)
migrate-down:
	sqlx migrate revert

# Check for compilation errors without producing an executable
check:
	cargo check

# Run clippy for linting
lint:
	cargo clippy

# Build for production (release mode)
build-release:
	cargo build --release

# Run in release mode
run-release:
	cargo run --release

# Format code
format:
	cargo fmt

# Check formatting without making changes
format-check:
	cargo fmt -- --check

# Run all checks (format, lint, test)
check-all: format-check lint test

# Development mode with hot reload (requires cargo-watch)
dev:
	cargo watch -x run

# Help command
help:
	@echo "Available commands:"
	@echo "  build         - Build the project"
	@echo "  run           - Run the project in debug mode"
	@echo "  test          - Run tests"
	@echo "  clean         - Clean build artifacts"
	@echo ""
	@echo "Migration commands (with dotenvx):"
	@echo "  migrate       - Run database migrations with dotenvx"
	@echo "  migrate-add   - Add new migration (requires name=<migration_name>)"
	@echo "  migrate-revert- Revert last migration with dotenvx"
	@echo "  prepare       - Prepare SQLx offline mode with dotenvx"
	@echo ""
	@echo "Migration commands (direct):"
	@echo "  migrate-up    - Run database migrations directly"
	@echo "  migrate-down  - Revert last migration directly"
	@echo ""
	@echo "Code quality commands:"
	@echo "  check         - Check for compilation errors"
	@echo "  lint          - Run clippy for linting"
	@echo "  format        - Format code"
	@echo "  format-check  - Check formatting"
	@echo "  check-all     - Run all checks (format, lint, test)"
	@echo ""
	@echo "Production commands:"
	@echo "  build-release - Build for production"
	@echo "  run-release   - Run in release mode"
	@echo ""
	@echo "Development commands:"
	@echo "  dev           - Run with hot reload (requires cargo-watch)"
	@echo ""
	@echo "Other commands:"
	@echo "  help          - Show this help message"

# Default target
default: help