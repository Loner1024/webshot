build-docker:
	cargo build -p service --release --target x86_64-unknown-linux-musl
	docker buildx build -t web_shot_service .
