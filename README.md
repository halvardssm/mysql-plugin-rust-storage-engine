# CrustDB - MariaDB/MySQL Rust Storage Engine

A POC storage engine for MariaDB (and MySQL).

## Architecture

The plugin entrypoint is in `src/cpp/plugin.c`. When compiling (see
`Makefile -> build`), it includes the headers from the database, and statically
links the rust code.

This generates a dynamic library in `target/plugin` that can be placed in the MariaDB plugins folder (we are using `/usr/lib/mysql/plugin` for the integration/e2e tests)

## Build

To build the code, you need to have `podman` or `docker` installed.

1. `make init`
   - This will initialize the submodules, setup the container for build, and
     builds MariaDB at the specified version.
2. `make build`
   - Builds the rust library as a static library
   - Builds the c plugin as a dynamic library, and links the rust library statically to the c plugin.
   - The final plugin can be found in `target/plugin`

### Updating MariaDB version

When updating the MariaDB version (make sure the checked out mariadb and the mariadb image are the (exact) same version), the includes folder needs to be generated anew. The headers are generated during the MariaDB compilation. It is assumed that you have previously ran `make init`

1. `make build-mariadb`
   - Builds the checked out version of MariaDB

## Tests

To run the tests, you can run `make cargo-test`
