CARGO_PROFILE?=dev

.PHONY: build test clean run

build:
	cargo build --profile ${CARGO_PROFILE}

run:
	cargo run --profile ${CARGO_PROFILE}

run-fake:
	cargo run -F fake_responses --profile ${CARGO_PROFILE}

clean:
	cargo clean

test:
	cargo test
