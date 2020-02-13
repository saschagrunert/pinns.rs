CARGO := cargo

all: build

.PHONY: build
build:
	$(CARGO) build

.PHONY: build-release
build-release:
	$(CARGO) build --release

all:
	$(CARGO) build

.PHONY: lint-clippy
lint-clippy:
	$(CARGO) clippy --all -- -D warnings

.PHONY: lint-rustfmt
lint-rustfmt:
	$(CARGO) fmt
	git diff --exit-code
