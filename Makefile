all: install
install:
	@mkdir -p /opt/cargo/duckdnsv6
	@cargo install --path . --root /opt/cargo
	@mkdir -p /usr/share/doc/duckdnsv6
	@cp doc/duckdnsv6.toml /usr/share/doc/duckdnsv6/duckdnsv6.toml.default
	@cp doc/duckdnsv6.toml /etc/
	@cp doc/duckdnsv6.toml /opt/cargo/duckdnsv6