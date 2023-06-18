build:
	cargo build

gen:
	make build
	sea-orm-cli generate entity -o ./src/entities

run:
	cargo run

