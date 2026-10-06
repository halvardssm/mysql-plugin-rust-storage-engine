# TODO

## Immediate blockers

- [ ] Fix compile error in `src/lib.rs` (`create_table` passes `*const u8`, handler expects `*const c_char` — E0308)
- [ ] Clean up compiler warnings: unused imports (`LazyLock`, `Mutex`, `OnceLock`, `RwLock`), unused parameters in exported extern fns
- [ ] Run `mise run init` to check out the MariaDB submodule (~1 GB) and generate headers — currently the submodule is empty, so the plugin cannot be linked
- [ ] Run `mise run build` and verify `target/plugin/libcrustdb.so` is produced
- [ ] Confirm the `cmake --build` step commented out in `build:mariadb` is not needed for header generation (or wire it back in if it is)

## Make the engine functional

The handlerton currently only sets `MYSQL_HANDLERTON_INTERFACE_VERSION` — none of the exported Rust functions are registered as callbacks, so they are dead code from MariaDB's perspective.

- [ ] Register handler callbacks in `st_mysql_storage_engine` / handler struct: `create`, `open`, `close`, `rnd_init`, `rnd_next`, `write_row`
- [ ] Set capability flags (`HA_*`) and table flags on the handlerton
- [ ] Implement `RustStorageHandler::open` / `close` / `create_table` / `drop_table`
- [ ] Implement full table scan: `rnd_init` / `rnd_next` returning rows in table format
- [ ] Implement `write_row` (insert); then `update_row` / `delete_row`
- [ ] Minimal viable milestone: `CREATE TABLE t (...) ENGINE=rust_storage`, `INSERT`, and a full-scan `SELECT` return the correct data
- [ ] Replace every `todo!()` across the FFI boundary with proper `HA_ERR_*` error codes — a Rust panic aborts the server process
- [ ] Review `println!` usage inside the engine — plugin code runs inside mysqld, prefer the server error log via `sql_print_information`/`sql_print_error` where possible

## Later engine work (not needed for the first milestone)

- [ ] Index support: `index_init` / `index_read` / `index_next` (`create_index` / `drop_index`)
- [ ] Transactions: `external_lock`, `start_stmt`, `begin` / `commit` / `rollback` (or declare the engine non-transactional explicitly)
- [ ] `info()` / `table_flags()` — statistics and capabilities for the optimizer
- [ ] Position-based access (`position` / `rnd_pos`) if needed for `ORDER BY` / `GROUP BY` optimization
- [ ] Define on-disk table format (current `create_table` only writes an `id,data\n` CSV header)

## Testing

- [ ] Harden `tests/load_plugin_test.rs`: assert on query results, not just that `mariadb --verbose` echoes the `INSTALL PLUGIN` command back
- [ ] Add test cases: `CREATE TABLE ... ENGINE=rust_storage`, `INSERT`, full-scan `SELECT`, error paths (e.g. table already exists, unknown table)
- [ ] Add rust unit tests for `RustStorageHandler` logic
- [ ] Add CI (GitHub Actions): `cargo check`/`clippy`/`fmt` plus the container build

## Housekeeping

- [ ] Pin the MariaDB submodule to the exact tag matching the `mariadb:11.4` test image (README warns about version drift between the checked-out sources and the image)
- [ ] Remove or wire the ~15 exported-but-unreferenced extern fns in `src/lib.rs` (`rnd_pos`, `position`, `index_read`, `commit_transaction`, ...)
- [ ] Decide on plugin naming consistency: plugin name `rust_storage`, library `libcrustdb.so`, crate `crustdb`, table test uses `crustdb.so`
- [ ] `mariadb-clean` (now `clean:mariadb`) is destructive — consider adding `confirm` in `mise.toml`
