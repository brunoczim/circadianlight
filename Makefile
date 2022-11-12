RUST_FILES = $(shell find src -type f -name '*.rs')

build_release: Cargo.toml Cargo.lock $(RUST_FILES)
	cargo build --release

install: build_release circadianlight.service
	@ set -e
	cp ./target/release/circadianlight /usr/bin/circadianlight
	cp ./circadianlight.service /etc/systemd/system/circadianlight.service

uninstall:
	@ set -e
	rm -f /usr/bin/circadianlight
	rm -f /etc/systemd/system/circadianlight.service
