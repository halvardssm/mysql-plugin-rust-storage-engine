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
    {(110406 << 8)};

maria_declare_plugin(crustdb){
    MYSQL_STORAGE_ENGINE_PLUGIN, /* int type; the plugin type (a MYSQL_XXX_PLUGIN value)   */
    &crustdb_storage_engine,     /* void *info; pointer to type-specific plugin descriptor   */
    "rust_storage",              /* const char *name; plugin name                                  */
    "Halvard Mørstad",           /* const char *author; plugin author (for SHOW PLUGINS)             */
    "Test DB.",                  /* const char *descr; general descriptive text (for SHOW PLUGINS ) */
    PLUGIN_LICENSE_GPL,          /* int license; the plugin license (PLUGIN_LICENSE_XXX)      */
    rust_storage_engine_init,    /* Plugin Init */
    rust_storage_engine_deinit,  /* Plugin Deinit */
    0x0100 /* 1.0 */,
    NULL,                          /* status variables                */
    NULL,                          /* system variables                */
    "1.0",                         /* string version */
    MariaDB_PLUGIN_MATURITY_STABLE /* maturity */
} maria_declare_plugin_end;