FEATURES := bevy/dynamic
CARGO_OPTS := --color always

check build run:
	cargo $(CARGO_OPTS) $@ --features="$(FEATURES)" -- $(args)

RELEASE_RUSTFLAGS := "-C link-arg=-s"

release:
	RUSTFLAGS=$(RELEASE_RUSTFLAGS) cargo build --release

release-run:
	RUSTFLAGS=$(RELEASE_RUSTFLAGS) cargo run --release

web-install-requirements:
	rustup target install wasm32-unknown-unknown
	cargo install basic-http-server
	cargo install \
		--force \
		--version 0.2.69 \
		wasm-bindgen-cli

web:
	cargo build \
		--target wasm32-unknown-unknown \
		--no-default-features \
		--features "web, bevy/bevy_gltf, bevy/bevy_winit, bevy/render, bevy/png"

	@-mkdir target/web 2> /dev/null || true

	wasm-bindgen \
		--out-dir target/web \
		--out-name wasm \
		--target web \
		--no-typescript \
		target/wasm32-unknown-unknown/debug/kod_jam.wasm

	cp index.html target/web/index.html
	cp -r assets target/web/


serve: web
	basic-http-server -x target/web/
