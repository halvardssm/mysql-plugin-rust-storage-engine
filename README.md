# CrustDB - MariaDB/MySQL Rust Storage Engine

A POC storage engine for MariaDB (and MySQL).

## Architecture

The plugin entrypoint is in `src/cpp/plugin.c`. When compiling (see
`Makefile -> build`), it includes the headers from the database, and statically
links the rust code.

## Build

To build the code, you need to have `podman` or `docker` installed.

1. `make init`
   - This will initialize the submodules, setup the container for build, and
     builds MariaDB at the specified version.
2. `make build`
   - Builds the rust library (static library), the c plugin (dynamic library),
     and links the rust library statically to the c plugin.
   - The plugin can be found in `target/plugin`

## Tests

To run the tests, you can run `make cargo-test`
