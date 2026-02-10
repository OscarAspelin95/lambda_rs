.PHONY: build-for-lambda docker-start docker-stop

build-for-lambda:
	@echo "Building..."
	@cargo lambda build --release --x86-64

docker-start:
	docker compose up -d

docker-stop:
	docker compose down

docker-prune:
	docker system prune --volumes -f
