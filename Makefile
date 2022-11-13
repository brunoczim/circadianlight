RUST_FILES = $(shell find src -type f -name '*.rs')

USER_SYSTEMD_DIR = "$(HOME)/.config/systemd/user"

build-release: Cargo.toml $(RUST_FILES)
	cargo build --release

install: build-release circadianlight.service
	@ set -e
	cargo install --path .
	cp ./circadianlight.service "$(USER_SYSTEMD_DIR)/circadianlight.service"
	systemctl --user enable circadianlight.service
	systemctl --user start circadianlight.service

uninstall:
	@ set -e
	systemctl --user stop circadianlight.service
	systemctl --user disable circadianlight.service
	rm -f "$(USER_SYSTEMD_DIR)/circadianlight.service"
	cargo uninstall circadianlight
