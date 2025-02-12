LIB_NAME=libcrustdb

build:
	podman build -v `pwd`:/wdir -t mariadb-crust --no-cache .
run: build kill
	podman run --rm -d -e MARIADB_ALLOW_EMPTY_ROOT_PASSWORD=true --name mariadb-crust --health-cmd='["healthcheck.sh","--connect","--innodb_initialized"]' --health-interval 10s --health-timeout 5s mariadb-crust
	@while [ "$$(podman inspect --format '{{.State.Health.Status}}' mariadb-crust)" != "healthy" ]; do sleep 1; done
	podman exec -ti mariadb-crust mariadb -e "INSTALL PLUGIN rust_storage SONAME 'crustdb.so';"
kill:
	podman kill mariadb-crust || true
	sleep 1
ssh:
	podman exec -ti mariadb-crust mariadb


build-cargo-debug:
	podman run --rm -v `pwd`:/code -w /code -t rust:latest cargo build
build-cargo-release:
	podman run --rm -v `pwd`:/code -w /code -t rust:latest cargo build --release
build-gcc-debug: build-cargo-debug
	mkdir -p target/plugin
	podman run --rm -v `pwd`:/code -w /code -t rust:latest gcc -DMYSQL_DYNAMIC_PLUGIN -DMYSQL_ABI_CHECK -fpic -shared \
		-o target/plugin/$(LIB_NAME).so -I mariadb/include -w src/cpp/plugin.c \
		target/debug/$(LIB_NAME).a \
		-lpthread -ldl
build-gcc-release: build-cargo-release
	mkdir -p target/plugin
	podman run --rm -v `pwd`:/code -w /code -t rust:latest gcc -DMYSQL_DYNAMIC_PLUGIN -DMYSQL_ABI_CHECK -fpic -shared \
		-o target/plugin/$(LIB_NAME).so -I mariadb/include -w src/cpp/plugin.c \
		target/release/$(LIB_NAME).a \
		-lpthread -ldl

cargo-test: build-gcc-debug
	cargo test -- --nocapture


oci_tt:
	podman build -v `pwd`/mariadb:/mariadb -t mariadb-server-build -f oci/mariadb.containerfile .
