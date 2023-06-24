build:
	cargo build

gen:
	make build
	sea-orm-cli generate entity -o ./src/generated/models

run:
	cargo run

