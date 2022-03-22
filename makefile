RFLAGS="-C link-arg=-s"

all: sim-token mock-dex

sim-token: contracts/sim-token
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RFLAGS) cargo build -p sim_token --target wasm32-unknown-unknown --release
	mkdir -p res
	cp target/wasm32-unknown-unknown/release/sim_token.wasm ./res/sim_token.wasm

mock-dex: contracts/mock-dex
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RFLAGS) cargo build -p mock_dex --target wasm32-unknown-unknown --release
	mkdir -p res
	cp target/wasm32-unknown-unknown/release/mock_dex.wasm ./res/mock_dex.wasm

clean:
	rm res/*.wasm

test:
	cargo test