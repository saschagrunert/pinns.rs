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
	podman run --rm -it -v $(shell pwd):/home/rust/src \
		ekidd/rust-musl-builder:1.39.0 \
		cargo build --release

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
