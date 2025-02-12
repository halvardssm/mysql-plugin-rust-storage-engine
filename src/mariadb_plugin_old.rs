// // #[repr(C)]
// // pub struct MariaPlugin {
// //     pub interface_version: i32,
// //     pub struct_size: usize,
// // }
// // // #[no_mangle]
// // // pub static _mysql_plugin_interface_version_: i32 = 260;

// // #[no_mangle]
// // pub static MARIA_PLUGIN_INTERFACE_VERSION: i32 = 260; // Replace with actual version

// // #[no_mangle]
// // pub static MARIA_SIZEOF_STRUCT_ST_PLUGIN: usize = std::mem::size_of::<MariaPlugin>();

// // #[macro_export]
// // macro_rules! declare_maria_plugin {
// //     ($name:ident) => {
// //         #[no_mangle]
// //         pub static mut builtin_maria_ $name _plugin_interface_version: i32 = MARIA_PLUGIN_INTERFACE_VERSION;

// //         #[no_mangle]
// //         pub static mut builtin_maria_ $name _sizeof_struct_st_plugin: usize = MARIA_SIZEOF_STRUCT_ST_PLUGIN;

// //         #[no_mangle]
// //         pub static builtin_maria_ $name _plugin: MariaPlugin = MariaPlugin {
// //             interface_version: MARIA_PLUGIN_INTERFACE_VERSION,
// //             struct_size: MARIA_SIZEOF_STRUCT_ST_PLUGIN,
// //         };
// //     };
// // }

// // #[macro_export]
// // macro_rules! declare_maria_plugin_end {
// //     () => {
// //         #[no_mangle]
// //         pub static maria_plugin_declarations_end: MariaPlugin = MariaPlugin {
// //             interface_version: 0,
// //             struct_size: 0,
// //         };
// //     };
// // }

// // #[macro_use]
// // extern crate lazy_static;

// // use std::ffi::c_void;

// // // Define plugin struct similar to `st_maria_plugin`
// // #[repr(C)]
// // pub struct MariaPlugin {
// //     field1: i32,
// //     field2: i32,
// //     field3: i32,
// //     field4: i32,
// //     field5: i32,
// //     field6: i32,
// //     field7: i32,
// //     field8: i32,
// //     field9: i32,
// //     field10: i32,
// //     field11: i32,
// //     field12: i32,
// //     field13: i32,
// // }

// // // Plugin interface version and struct size
// // #[no_mangle]
// // pub static MARIA_PLUGIN_INTERFACE_VERSION: i32 = 1; // Adjust as needed

// // #[no_mangle]
// // pub static MARIA_SIZEOF_STRUCT_ST_PLUGIN: usize = std::mem::size_of::<MariaPlugin>();

// // // Macro for declaring MariaDB plugin
// // macro_rules! maria_declare_plugin {
// //     ($name:ident) => {
// //         #[no_mangle]
// //         pub static builtin_maria_{}_plugin_interface_version: i32 = MARIA_PLUGIN_INTERFACE_VERSION;

// //         #[no_mangle]
// //         pub static builtin_maria_{}_sizeof_struct_st_plugin: usize = MARIA_SIZEOF_STRUCT_ST_PLUGIN;

// //         #[no_mangle]
// //         pub static builtin_maria_{}_plugin: [MariaPlugin; 1] = [MariaPlugin {
// //             field1: 0,
// //             field2: 0,
// //             field3: 0,
// //             field4: 0,
// //             field5: 0,
// //             field6: 0,
// //             field7: 0,
// //             field8: 0,
// //             field9: 0,
// //             field10: 0,
// //             field11: 0,
// //             field12: 0,
// //             field13: 0,
// //         }];
// //     };
// // }

// // // End marker for plugin array
// // pub const MARIA_DECLARE_PLUGIN_END: MariaPlugin = MariaPlugin {
// //     field1: 0,
// //     field2: 0,
// //     field3: 0,
// //     field4: 0,
// //     field5: 0,
// //     field6: 0,
// //     field7: 0,
// //     field8: 0,
// //     field9: 0,
// //     field10: 0,
// //     field11: 0,
// //     field12: 0,
// //     field13: 0,
// // };

// // struct st_mysql_plugin
// // {
// //     int type;             /**< the plugin type (a MYSQL_XXX_PLUGIN value)   */
// //     void *info;           /**< pointer to type-specific plugin descriptor   */
// //     const char *name;     /**< plugin name                                  */
// //     const char *author;   /**< plugin author (for I_S.PLUGINS)              */
// //     const char *descr;    /**< general descriptive text (for I_S.PLUGINS)   */
// //     int license;          /**< the plugin license (PLUGIN_LICENSE_XXX)      */
// //     /**
// //       The function to invoke when plugin is loaded. Plugin
// //       initialisation done here should defer any ALTER TABLE queries to
// //       after the ddl recovery is done, in the signal_ddl_recovery_done()
// //       callback called by ha_signal_ddl_recovery_done().
// //     */
// //     int (*init)(void *);
// //     int (*deinit)(void *);/**< the function to invoke when plugin is unloaded */
// //     unsigned int version; /**< plugin version (for I_S.PLUGINS)               */
// //     struct st_mysql_show_var *status_vars;
// //     struct st_mysql_sys_var **system_vars;
// //     void * __reserved1;   /**< reserved for dependency checking               */
// //     unsigned long flags;  /**< flags for plugin */
// //   };

// use std::ffi::{c_void, CString};

// macro_rules! maria_declare_plugin {
//     () => {
//         use std::ffi::{c_void, CString};
//         #[no_mangle]
//         pub static _mysql_plugin_interface_version_: i32 = 260;
//         pub static _maria_plugin_interface_version_: i32 = 271;

//         pub static fn mysql_declare_plugin()

//         // pub static _mysql_plugin_declarations_: StMysqlPlugin = StMysqlPlugin {
//         //     plugin_type: 1,
//         // };
//         // pub static _maria_plugin_declarations_: StMysqlPlugin = StMysqlPlugin {
//         //     plugin_type: 1,
//         // };

//         #[repr(C)]
//         pub struct StMysqlPlugin {
//             pub plugin_type: i32
//         }

//         // #[repr(C)]
//         // pub struct StMysqlPlugin {
//         //     pub plugin_type: i32, // `type` is a reserved keyword in Rust, so we rename it to `plugin_type`
//         //     pub info: *mut c_void,
//         //     pub name: *const i8,   // C-style string (null-terminated)
//         //     pub author: *const i8, // C-style string (null-terminated)
//         //     pub descr: *const i8,  // C-style string (null-terminated)
//         //     pub license: i32,      // Plugin license (PLUGIN_LICENSE_XXX)

//         //     // Function pointers for plugin lifecycle management
//         //     pub init: Option<extern "C" fn(*mut c_void) -> i32>,
//         //     pub deinit: Option<extern "C" fn(*mut c_void) -> i32>,

//         //     pub version: u32, // Plugin version (for INFORMATION_SCHEMA.PLUGINS)
//         //     pub status_vars: *mut StMysqlShowVar, // Pointer to status variables
//         //     pub system_vars: *mut *mut StMysqlSysVar, // Pointer to system variables
//         //     pub reserved1: *mut c_void, // Reserved for dependency checking
//         //     pub flags: u64,   // Plugin flags
//         // }

//         // // Define placeholder structs for system variables and status variables
//         // #[repr(C)]
//         // pub struct StMysqlShowVar {
//         //     // Define this according to MySQL/MariaDB expectations
//         // }

//         // #[repr(C)]
//         // pub struct StMysqlSysVar {
//         //     // Define this according to MySQL/MariaDB expectations
//         // }
//     };
// }

// use std::ffi::c_void;

// #[repr(C)]
// pub struct StMysqlPlugin {
//     pub plugin_type: i32,
//     pub info: *mut c_void,
//     pub name: *const i8,
//     pub author: *const i8,
//     pub descr: *const i8,
//     pub license: i32,
//     pub init: Option<extern "C" fn(*mut c_void) -> i32>,
//     pub deinit: Option<extern "C" fn(*mut c_void) -> i32>,
//     pub version: u32,
//     pub status_vars: *mut StMysqlShowVar,
//     pub system_vars: *mut *mut StMysqlSysVar,
//     pub reserved1: *mut c_void,
//     pub flags: u64,
// }

// #[repr(C)]
// pub struct StMariaPlugin {
//     pub plugin_type: i32,
//     pub info: *mut c_void,
//     pub name: *const i8,
//     pub author: *const i8,
//     pub descr: *const i8,
//     pub license: i32,
//     pub init: Option<extern "C" fn(*mut c_void) -> i32>,
//     pub deinit: Option<extern "C" fn(*mut c_void) -> i32>,
//     pub version: u32,
//     pub status_vars: *mut StMysqlShowVar,
//     pub system_vars: *mut *mut StMysqlSysVar,
//     pub reserved1: *mut c_void,
//     pub flags: u64,
// }

// // Define placeholders for system variables and status variables
// #[repr(C)]
// pub struct StMysqlShowVar {}

// #[repr(C)]
// pub struct StMysqlSysVar {}

// // Macro to declare MySQL plugin
// macro_rules! mysql_declare_plugin {
//     ($name:ident) => {
//         #[no_mangle]
//         pub static mut builtin_{}_plugin_interface_version: i32 = _mysql_plugin_interface_version_;

//         #[no_mangle]
//         pub static mut builtin_{}_sizeof_struct_st_plugin: usize = std::mem::size_of::<StMysqlPlugin>();

//         #[no_mangle]
//         pub static mut builtin_{}_plugin: [StMysqlPlugin; 1] = [StMysqlPlugin {
//             plugin_type: 0,
//             info: std::ptr::null_mut(),
//             name: std::ptr::null(),
//             author: std::ptr::null(),
//             descr: std::ptr::null(),
//             license: 0,
//             init: None,
//             deinit: None,
//             version: 0,
//             status_vars: std::ptr::null_mut(),
//             system_vars: std::ptr::null_mut(),
//             reserved1: std::ptr::null_mut(),
//             flags: 0,
//         }];
//     };
// }

// // Macro to declare MariaDB plugin
// macro_rules! maria_declare_plugin {
//     ($name:ident) => {
//         #[no_mangle]
//         pub static mut builtin_maria_{}_plugin_interface_version: i32 = _maria_plugin_interface_version_;

//         #[no_mangle]
//         pub static mut builtin_maria_{}_sizeof_struct_st_plugin: usize = std::mem::size_of::<StMariaPlugin>();

//         #[no_mangle]
//         pub static mut builtin_maria_{}_plugin: [StMariaPlugin; 1] = [StMariaPlugin {
//             plugin_type: 0,
//             info: std::ptr::null_mut(),
//             name: std::ptr::null(),
//             author: std::ptr::null(),
//             descr: std::ptr::null(),
//             license: 0,
//             init: None,
//             deinit: None,
//             version: 0,
//             status_vars: std::ptr::null_mut(),
//             system_vars: std::ptr::null_mut(),
//             reserved1: std::ptr::null_mut(),
//             flags: 0,
//         }];
//     };
// }

// // Plugin end marker
// #[no_mangle]
// pub static mut mysql_declare_plugin_end: StMysqlPlugin = StMysqlPlugin {
//     plugin_type: 0,
//     info: std::ptr::null_mut(),
//     name: std::ptr::null(),
//     author: std::ptr::null(),
//     descr: std::ptr::null(),
//     license: 0,
//     init: None,
//     deinit: None,
//     version: 0,
//     status_vars: std::ptr::null_mut(),
//     system_vars: std::ptr::null_mut(),
//     reserved1: std::ptr::null_mut(),
//     flags: 0,
// };

// #[no_mangle]
// pub static mut maria_declare_plugin_end: StMariaPlugin = StMariaPlugin {
//     plugin_type: 0,
//     info: std::ptr::null_mut(),
//     name: std::ptr::null(),
//     author: std::ptr::null(),
//     descr: std::ptr::null(),
//     license: 0,
//     init: None,
//     deinit: None,
//     version: 0,
//     status_vars: std::ptr::null_mut(),
//     system_vars: std::ptr::null_mut(),
//     reserved1: std::ptr::null_mut(),
//     flags: 0,
// };

use std::ffi::c_void;
use std::os::raw::{c_char, c_int};

// Define plugin structs similar to MySQL/MariaDB
#[repr(C)]
pub struct StMysqlPlugin {
    pub plugin_type: i32, 
    pub info: *mut c_void,
    pub name: *const u8,    
    pub author: *const i8,  
    pub descr: *const i8,   
    pub license: i32,       

    pub init: Option<extern "C" fn(*mut c_void) -> i32>,
    pub deinit: Option<extern "C" fn(*mut c_void) -> i32>,

    pub version: u32, 
    pub status_vars: *mut StMysqlShowVar,      
    pub system_vars: *mut *mut StMysqlSysVar,  
    pub reserved1: *mut c_void,                
    pub flags: u64,                            
}

// Placeholder structs
#[repr(C)]
pub struct StMysqlShowVar {}

#[repr(C)]
pub struct StMysqlSysVar {}

// Declare exported variables
#[no_mangle]
pub static _mysql_plugin_interface_version_: i32 = 260;
#[no_mangle]
pub static _maria_plugin_interface_version_: i32 = 271;

#[no_mangle]
pub static _mysql_sizeof_struct_st_plugin_: usize = std::mem::size_of::<StMysqlPlugin>();
#[no_mangle]
pub static _maria_sizeof_struct_st_plugin_: usize = std::mem::size_of::<StMysqlPlugin>();

// // Plugin metadata
// #[no_mangle]
// pub static rust_storage_plugin: MysqlStorageEngine = MysqlStorageEngine {
//     interface_version: 0x0100_0000,
//     name: b"rust_storage\0".as_ptr() as *const c_char,
//     author: b"SimplyUndoable\0".as_ptr() as *const c_char,
//     description: b"Rust-based MySQL/MariaDB storage engine\0".as_ptr() as *const c_char,
//     license: b"GPL\0".as_ptr() as *const c_char,
//     init: Some(rust_storage_engine_init),
//     deinit: Some(rust_storage_engine_deinit),
// };


#[no_mangle]
pub static mut _mysql_plugin_declarations_: StMysqlPlugin = StMysqlPlugin {
    plugin_type: 0,
    info: std::ptr::null_mut(),
    name: b"rust_storage\0".as_ptr(),
    author: std::ptr::null(),
    descr: std::ptr::null(),
    license: 0,
    init: None,
    deinit: None,
    version: 1,
    status_vars: std::ptr::null_mut(),
    system_vars: std::ptr::null_mut(),
    reserved1: std::ptr::null_mut(),
    flags: 0,
};

#[no_mangle]
pub static mut _maria_plugin_declarations_: StMysqlPlugin= StMysqlPlugin {
    plugin_type: 0,
    info: std::ptr::null_mut(),
    name: b"rust_storage\0".as_ptr(),
    author: std::ptr::null(),
    descr: std::ptr::null(),
    license: 0,
    init: None,
    deinit: None,
    version: 1,
    status_vars: std::ptr::null_mut(),
    system_vars: std::ptr::null_mut(),
    reserved1: std::ptr::null_mut(),
    flags: 0,
};

// Macro to declare a MySQL plugin
macro_rules! mysql_declare_plugin {
    ($name:ident) => {
        #[no_mangle]
        pub static builtin_ $name _plugin_interface_version: i32 = 260;

        #[no_mangle]
        pub static builtin_ $name _sizeof_struct_st_plugin: usize = std::mem::size_of::<StMysqlPlugin>();

        #[no_mangle]
        pub static builtin_ $name _plugin: [StMysqlPlugin; 1] = [StMysqlPlugin {
            plugin_type: 0,
            info: std::ptr::null_mut(),
            name: std::ptr::null(),
            author: std::ptr::null(),
            descr: std::ptr::null(),
            license: 0,
            init: None,
            deinit: None,
            version: 1,
            status_vars: std::ptr::null_mut(),
            system_vars: std::ptr::null_mut(),
            reserved1: std::ptr::null_mut(),
            flags: 0,
        }];
    };
}

// Macro to declare a MariaDB plugin
macro_rules! maria_declare_plugin {
    ($name:ident) => {
        #[no_mangle]
        pub static builtin_maria_ $name _plugin_interface_version: i32 = 271;

        #[no_mangle]
        pub static builtin_maria_ $name _sizeof_struct_st_plugin: usize = std::mem::size_of::<StMysqlPlugin>();

        #[no_mangle]
        pub static builtin_maria_ $name _plugin: [StMysqlPlugin; 1] = [StMysqlPlugin {
            plugin_type: 0,
            info: std::ptr::null_mut(),
            name: std::ptr::null(),
            author: std::ptr::null(),
            descr: std::ptr::null(),
            license: 0,
            init: None,
            deinit: None,
            version: 1,
            status_vars: std::ptr::null_mut(),
            system_vars: std::ptr::null_mut(),
            reserved1: std::ptr::null_mut(),
            flags: 0,
        }];
    };
}

// End marker for plugin array
pub const MYSQL_DECLARE_PLUGIN_END: StMysqlPlugin = StMysqlPlugin {
    plugin_type: 0,
    info: std::ptr::null_mut(),
    name: std::ptr::null(),
    author: std::ptr::null(),
    descr: std::ptr::null(),
    license: 0,
    init: None,
    deinit: None,
    version: 1,
    status_vars: std::ptr::null_mut(),
    system_vars: std::ptr::null_mut(),
    reserved1: std::ptr::null_mut(),
    flags: 0,
};

pub const MARIA_DECLARE_PLUGIN_END: StMysqlPlugin = MYSQL_DECLARE_PLUGIN_END;
