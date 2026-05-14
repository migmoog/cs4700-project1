all:
	apt update && apt install -y cargo
	cargo build
	cp target/debug/project1 client
