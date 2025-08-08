# StageX Rust Builder for eth_transfer_enclave
BINARY_NAME := eth_transfer_enclave
IMAGE_TAG := eth-simulator-stagex:latest

.PHONY: help
help:
	@echo "StageX Rust Builder for eth_transfer_enclave"
	@echo "Available targets:"
	@echo "  build         - Build the eth_transfer_enclave binary using StageX"
	@echo "  extract       - Extract the built binary from container"
	@echo "  run           - Run the built container"
	@echo "  demo          - Build, extract, and test the binary"
	@echo "  clean         - Clean up containers and images"

.PHONY: build
build:
	docker build \
		--file Containerfile \
		--tag $(IMAGE_TAG) \
		.

.PHONY: extract
extract:
	@echo "Extracting binary from container..."
	docker create --name temp-extract $(IMAGE_TAG)
	docker cp temp-extract:/app ./$(BINARY_NAME)
	docker rm temp-extract
	@echo "Binary extracted to ./$(BINARY_NAME)"

.PHONY: run
run:
	docker run --rm $(IMAGE_TAG) --help

.PHONY: clean
clean:
	docker rmi $(IMAGE_TAG) || true
	docker system prune -f
	rm -f ./$(BINARY_NAME)

.PHONY: demo
demo: build extract
	@echo "Testing extracted binary:"
	./$(BINARY_NAME) --help || echo "Binary extracted successfully but may need runtime arguments"
	@echo "Demo complete! Binary available at ./$(BINARY_NAME)"