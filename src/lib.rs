mod storage_handler;

use std::os::raw::c_int;
use std::sync::{LazyLock, Mutex};
use storage_handler::RustStorageHandler;

static HANDLER: LazyLock<Mutex<RustStorageHandler>> =
    LazyLock::new(|| Mutex::new(RustStorageHandler::new()));

// Define the storage engine metadata
// pub extern "C" fn mysql_plugin_init() -> i32 {
#[no_mangle]
pub extern "C" fn rust_storage_engine_init() -> c_int {
    println!("Rust storage engine initializing...");

    HANDLER.lock().unwrap().init();

    0 // Return 0 on success
}

// Define the handler struct for managing tables
// pub extern "C" fn mysql_plugin_deinit() -> i32 {
#[no_mangle]
pub extern "C" fn rust_storage_engine_deinit() -> c_int {
    println!("Rust storage engine shutting down...");

    HANDLER.lock().unwrap().deinit();

    0 // Return 0 on success
}

#[no_mangle]
pub extern "C" fn create_table(name: *const u8) -> i32 {
    HANDLER.lock().unwrap().create_table(name);
    0
}

#[no_mangle]
pub extern "C" fn drop_table(name: *const u8) -> i32 {
    println!("Dropping table");
    0
}

#[no_mangle]
pub extern "C" fn open_table(name: *const u8) -> i32 {
    println!("Opening table");
    0
}

#[no_mangle]
pub extern "C" fn close_table(name: *const u8) -> i32 {
    println!("Closing table");
    0
}

#[no_mangle]
pub extern "C" fn rnd_init() -> i32 {
    println!("Initializing table scan");
    0
}

#[no_mangle]
pub extern "C" fn rnd_next() -> i32 {
    println!("Retrieving next row");
    0
}

#[no_mangle]
pub extern "C" fn rnd_pos() -> i32 {
    println!("Retrieving row by position");
    0
}

#[no_mangle]
pub extern "C" fn position() -> i32 {
    println!("Returning row position");
    0
}

#[no_mangle]
pub extern "C" fn index_read() -> i32 {
    println!("Reading row by index lookup");
    0
}

#[no_mangle]
pub extern "C" fn index_next() -> i32 {
    println!("Reading next row from index scan");
    0
}

#[no_mangle]
pub extern "C" fn write_row() -> i32 {
    println!("Inserting new row");
    0
}

#[no_mangle]
pub extern "C" fn update_row() -> i32 {
    println!("Updating existing row");
    0
}

#[no_mangle]
pub extern "C" fn delete_row() -> i32 {
    println!("Deleting row");
    0
}

#[no_mangle]
pub extern "C" fn index_init() -> i32 {
    println!("Initializing index scan");
    0
}

#[no_mangle]
pub extern "C" fn index_end() -> i32 {
    println!("Ending index scan");
    0
}

#[no_mangle]
pub extern "C" fn begin_transaction() -> i32 {
    println!("Beginning transaction");
    0
}

#[no_mangle]
pub extern "C" fn commit_transaction() -> i32 {
    println!("Committing transaction");
    0
}

#[no_mangle]
pub extern "C" fn rollback_transaction() -> i32 {
    println!("Rolling back transaction");
    0
}

#[no_mangle]
pub extern "C" fn external_lock() -> i32 {
    println!("Handling external lock");
    0
}

#[no_mangle]
pub extern "C" fn start_stmt() -> i32 {
    println!("Starting statement");
    0
}

#[no_mangle]
pub extern "C" fn info() -> i32 {
    println!("Providing storage engine info");
    0
}

#[no_mangle]
pub extern "C" fn table_flags() -> i32 {
    println!("Returning storage engine capabilities");
    0
}
