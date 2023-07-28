prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p nft-cep47 --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/nft-cep47.wasm 2>/dev/null | true

