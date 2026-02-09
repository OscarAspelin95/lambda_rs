.PHONY: build-for-lambda

build-for-lambda:
	@echo "Building..."
	@cargo lambda build --release --arm64 --output-format "Zip"
