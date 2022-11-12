RUST_FILES = $(shell find src -type f -name '*.rs')

USER_BIN_DIR = "$(HOME)/.local/bin"
USER_SYSTEMD_DIR = "$(HOME)/.config/systemd/user"

build-release: Cargo.toml $(RUST_FILES)
	cargo build --release

install: build-release circadianlight.service
	@ set -e
	mkdir -p ~/.local/bin
	cp ./target/release/circadianlight "$(USER_BIN_DIR)/circadianlight"
	cp ./circadianlight.service "$(USER_SYSTEMD_DIR)/circadianlight.service"

uninstall:
	@ set -e
	rm -f "$(USER_BIN_DIR)/circadianlight"
	rm -f "$(USER_SYSTEMD_DIR)/circadianlight.service"
