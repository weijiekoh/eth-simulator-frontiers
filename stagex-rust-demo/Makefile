# StageX Rust Builder Demo Makefile
BINARY_NAME ?= hello-stagex
IMAGE_TAG ?= stagex-rust-demo:latest

.PHONY: help
help:
	@echo "StageX Rust Builder Demo"
	@echo "Available targets:"
	@echo "  build-basic    - Build using basic Containerfile"
	@echo "  build-flexible - Build using flexible Containerfile with custom binary name"
	@echo "  run           - Run the built container"
	@echo "  extract-binary - Extract the built binary from container"
	@echo "  clean         - Clean up containers and images"
	@echo ""
	@echo "Variables:"
	@echo "  BINARY_NAME   - Name of the binary to build (default: hello-stagex)"
	@echo "  IMAGE_TAG     - Container image tag (default: stagex-rust-demo:latest)"

.PHONY: build-basic
build-basic:
	docker build \
		--file Containerfile \
		--tag $(IMAGE_TAG) \
		example/

.PHONY: build-flexible
build-flexible:
	docker build \
		--file Containerfile.flexible \
		--build-arg BINARY_NAME=$(BINARY_NAME) \
		--tag $(IMAGE_TAG) \
		example/

.PHONY: run
run:
	docker run --rm $(IMAGE_TAG) StageX

.PHONY: extract-binary
extract-binary:
	@echo "Extracting binary from container..."
	docker create --name temp-extract $(IMAGE_TAG)
	docker cp temp-extract:/app ./$(BINARY_NAME)
	docker rm temp-extract
	@echo "Binary extracted to ./$(BINARY_NAME)"
	@echo "Testing extracted binary:"
	./$(BINARY_NAME) "Extracted Binary"

.PHONY: clean
clean:
	docker rmi $(IMAGE_TAG) || true
	docker system prune -f
	rm -f ./$(BINARY_NAME)

.PHONY: demo
demo: build-flexible run extract-binary
	@echo "Demo complete!"