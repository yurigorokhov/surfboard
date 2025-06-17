CARGO_PROFILE?=dev

.PHONY: build test clean run

build:
	cd surfboard && cargo build --profile ${CARGO_PROFILE}; cd -

run:
	cd surfboard && cargo run --profile ${CARGO_PROFILE}; cd -

clean:
	cd surfboard && cargo clean; cd -
	cd surfboard_lib && cargo clean; cd -

test:
	cd surfboard_lib && cargo test; cd -
