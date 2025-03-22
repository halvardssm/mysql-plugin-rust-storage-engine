LIB_NAME=libcrustdb
CONTAINER_CMD:=$(shell command -v podman || command -v docker)

help:
	@echo "Usage: make <target>"
	@echo ""
	@echo "WARNING: podman or docker is required for most commands"
	@echo ""
	@echo "Targets:"
	@echo "  help           - Show this help message"
	@echo "  init           - Initialize the project (checkout submodules, build mariadb and generate headers)"
	@echo "  build          - Build the plugin (builds the plugin in debug mode and outputs to target/plugin)"
	@echo "  build-mariadb  - Build MariaDB and generate headers"
	@echo "  test           - Run tests"

init:
	git submodule update --init --recursive
	@if [ -z "$(CONTAINER_CMD)" ]; then echo "Error: Neither podman nor docker is available"; exit 1; fi;
	@make build-mariadb

build-cargo-debug:
	$(CONTAINER_CMD) run --rm -v `pwd`:/code -w /code -t rust:latest cargo build
build-cargo-release:
	$(CONTAINER_CMD) run --rm -v `pwd`:/code -w /code -t rust:latest cargo build --release
build-gcc-debug: build-cargo-debug
	mkdir -p target/plugin
	$(CONTAINER_CMD) run --rm -v `pwd`:/code -w /code -t rust:latest gcc -DMYSQL_DYNAMIC_PLUGIN -DMYSQL_ABI_CHECK -fpic -shared \
		-o target/plugin/$(LIB_NAME).so -I mariadb/include -w src/cpp/plugin.c \
		target/debug/$(LIB_NAME).a \
		-lpthread -ldl
build-gcc-release: build-cargo-release
	mkdir -p target/plugin
	$(CONTAINER_CMD) run --rm -v `pwd`:/code -w /code -t rust:latest gcc -DMYSQL_DYNAMIC_PLUGIN -DMYSQL_ABI_CHECK -fpic -shared \
		-o target/plugin/$(LIB_NAME).so -I mariadb/include -w src/cpp/plugin.c \
		target/release/$(LIB_NAME).a \
		-lpthread -ldl

build: build-gcc-debug

cargo-test: build-gcc-debug
	cargo test -- --nocapture

.PHONY: test
test: cargo-test

build-mariadb:
	$(CONTAINER_CMD) build -t mariadb-server-build -f oci/mariadb-builder.containerfile oci
	$(CONTAINER_CMD) run -v `pwd`:/code -t mariadb-server-build cmake . -DCMAKE_BUILD_TYPE=Debug -DMYSQL_MAINTAINER_MODE=OFF
# 	$(CONTAINER_CMD) run -v `pwd`:/code -t mariadb-server-build cmake --build .

mariadb-clean:
	cd mariadb && git clean -xffd && git submodule foreach --recursive git clean -xffd
