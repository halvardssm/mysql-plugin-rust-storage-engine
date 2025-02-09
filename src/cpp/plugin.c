#include <stdlib.h>
#include <ctype.h>
#include <string.h>
#include <stdarg.h>
#include <mysql.h>
#include <mysql/plugin.h>

// rust function declarations
extern int rust_storage_engine_init();
extern int rust_storage_engine_deinit();

static struct st_mysql_storage_engine crustdb_storage_engine =
    {MYSQL_HANDLERTON_INTERFACE_VERSION};

maria_declare_plugin(crustdb){
    MYSQL_STORAGE_ENGINE_PLUGIN,
    &crustdb_storage_engine,
    "RustStorage",
    "MySQL AB",
    "Test DB.",
    PLUGIN_LICENSE_GPL,
    rust_storage_engine_init, /* Plugin Init */
    rust_storage_engine_deinit, /* Plugin Deinit */
    0x0100 /* 1.0 */,
    NULL,                          /* status variables                */
    NULL,                          /* system variables                */
    "1.0",                         /* string version */
    MariaDB_PLUGIN_MATURITY_STABLE /* maturity */
} maria_declare_plugin_end;