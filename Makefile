FEATURES := bevy/dynamic
CARGO_OPTS := --color always

check build run:
	cargo $(CARGO_OPTS) $@ --features="$(FEATURES)" -- $(args)

RELEASE_RUSTFLAGS := "-C link-arg=-s"

release:
	RUSTFLAGS=$(RELEASE_RUSTFLAGS) cargo build --release

release-run:
	RUSTFLAGS=$(RELEASE_RUSTFLAGS) cargo run --release

