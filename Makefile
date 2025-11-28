.PHONY: help build run test fmt lint clean check

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build the project in release mode
	cargo build --release

run: ## Run the server in development mode
	cargo run

test: ## Run all tests
	cargo test

fmt: ## Format the code
	cargo fmt

lint: ## Run clippy linter
	cargo clippy -- -D warnings

check: ## Check if code compiles
	cargo check

clean: ## Clean build artifacts
	cargo clean

all: fmt lint test build ## Format, lint, test, and build

