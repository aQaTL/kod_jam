FEATURES := bevy/dynamic
CARGO_OPTS := --color always

check build run:
	cargo $(CARGO_OPTS) $@ --features="$(FEATURES)"

release:
	RUSTFLAGS="-C link-arg=-s" cargo build --release
