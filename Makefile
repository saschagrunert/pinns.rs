CARGO := cargo

all: build

.PHONY: build
build:
	$(CARGO) build

.PHONY: build-release
build-release:
	RUSTFLAGS='-C link-arg=-s' \
	$(CARGO) build --release

.PHONY: build-static
build-static:
	sudo rm -rf target
	podman run --rm -it \
		-v $(shell pwd):/home/rust/src \
		ekidd/rust-musl-builder:latest \
		bash -c \
			"sudo mkdir target && \
			 sudo chown $$(id -u):$$(id -g) target && \
			 RUSTFLAGS='-C link-arg=-s' cargo build --release"
	sudo chown -R $(shell id -u):$(shell id -g) target

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: lint-clippy
lint-clippy:
	$(CARGO) clippy --all -- -D warnings

.PHONY: lint-rustfmt
lint-rustfmt:
	$(CARGO) fmt
	git diff --exit-code
