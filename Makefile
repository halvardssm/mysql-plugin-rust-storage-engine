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
