format:
	cargo fmt

lint:
	cargo clippy

setup:
	sudo apt install libpq-dev -y
	cargo install diesel_cli --no-default-features --features postgres
	cargo build
	cp .env.copy .env