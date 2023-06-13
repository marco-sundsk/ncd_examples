RFLAGS="-C link-arg=-s"

all: sim-token mock-dex

test: cchl
ifdef TF
	RUSTFLAGS=$(RFLAGS) cargo test -p cross-contract-high-level --test $(TF) -- --nocapture
else
	RUSTFLAGS=$(RFLAGS) cargo test -p cross-contract-high-level --tests -- --nocapture
endif

cchl: contracts/cchl
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RFLAGS) cargo build -p cross-contract-high-level --target wasm32-unknown-unknown --release
	mkdir -p res
	cp target/wasm32-unknown-unknown/release/cross_contract_high_level.wasm ./res/cross_contract_high_level.wasm

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
	cargo clean
	rm res/*.wasm
