RUST_FILES = $(shell find src -type f -name '*.rs')

ROOT_BIN_DIR = /usr/bin
ROOT_SYSTEMD_DIR = /etc/systemd/system
USER_BIN_DIR = ~/.local/bin
USER_SYSTEMD_DIR = ~/.config/systemd/user

build-release: Cargo.toml Cargo.lock $(RUST_FILES)
	cargo build --release

install-user: build-release circadianlight-user.service
	@ set -e
	mkdir -p ~/.local/bin
	cp ./target/release/circadianlight $(USER_BIN_DIR)/circadianlight
	cp ./circadianlight-user.service \
		$(USER_SYSTEMD_DIR)/circadianlight-user.service

uninstall-user:
	@ set -e
	rm -f $(USER_BIN_DIR)/circadianlight
	rm -f $(USER_SYSTEMD_DIR)/circadianlight-user.service

install-root: build-release circadianlight.service
	@ set -e
	cp ./target/release/circadianlight $(ROOT_BIN_DIR)/circadianlight
	cp ./circadianlight.service $(ROOT_SYSTEMD_DIR)/circadianlight.service

uninstall-root:
	@ set -e
	rm -f $(ROOT_BIN_DIR)/circadianlight
	rm -f $(ROOT_SYSTEMD_DIR)/circadianlight.service
