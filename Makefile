# Makefile for Rust project using Cargo

# Build the project with all features enabled in release mode
.PHONY: build
build:
	cargo build --release --all-features

# Update dependencies to their latest compatible versions
.PHONY: update
update:
	cargo update

# Run the project with all features enabled in release mode
.PHONY: run
run:
	cargo run --release --all-features

# Run all tests with all features enabled
.PHONY: test
test:
	cargo test --all-features

# Run benchmarks with all features enabled
.PHONY: bench
bench:
	cargo bench --all-features

# Run Clippy linter with nightly toolchain, fixing issues automatically
# and applying strict linting rules
.PHONY: clippy
clippy:
	cargo +nightly clippy --fix \
		--all-targets \
		--all-features \
		--allow-dirty \
		--allow-staged \
		-- -D warnings \
		-W clippy::pedantic \
		-W clippy::nursery \
		-W clippy::unwrap_used \
		-W clippy::expect_used

# Format the code using rustfmt with nightly toolchain
.PHONY: fmt
fmt:
	cargo +nightly fmt

# Generate documentation for all crates and open it in the browser
.PHONY: doc
doc:
	cargo +nightly doc --all-features --no-deps --open

# Run pre-commit hooks on all files
.PHONY: pre-commit
pre-commit:
	pre-commit run --all-files
