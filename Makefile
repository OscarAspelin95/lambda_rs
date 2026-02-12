.PHONY: build-for-lambda format-lint-fix deploy docker-start docker-stop

build-for-lambda:
	@echo "Building..."
	@cargo lambda build --release --x86-64

format-lint-fix:
	@cargo fmt --all
	@cargo clippy --fix --all --allow-dirty

deploy:
	cargo lambda build --release --x86-64 \
	&& cd terraform \
	&& terraform plan \
	&& terraform apply -auto-approve

docker-start:
	docker compose up -d

docker-stop:
	docker compose down

docker-prune:
	docker system prune --volumes -f
