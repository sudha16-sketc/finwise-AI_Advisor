default: build

all: test

test:
	cargo test --lib

build:
	stellar contract build
	@echo "✓ Soroban contract built successfully"
	@ls -l target/wasm32v1-none/release/*.wasm

build-release:
	cargo build --target wasm32-unknown-unknown --release
	@echo "✓ WASM binary ready for deployment"

fmt:
	cargo fmt --all

clean:
	cargo clean

deploy-testnet:
	@echo "To deploy the contract to testnet, run:"
	@echo "stellar contract deploy --wasm target/wasm32v1-none/release/piggybank_soroban.wasm --network testnet"
