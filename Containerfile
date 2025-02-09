FROM rust:latest as crustdb

WORKDIR /wdir
VOLUME [ "/wdir" ]

ADD mariadb-include /mariadb-include

RUN cargo build

RUN ls /wdir/target/debug

# RUN cp /wdir/target/debug/libcrustdb.so /libcrustdb.so

RUN gcc -DMYSQL_DYNAMIC_PLUGIN -DMYSQL_ABI_CHECK -fpic -shared \
-o /libcrustdb.so -I /mariadb-include -w src/cpp/plugin.c \
# target/debug/libcrustdb.a \
-lpthread -ldl

# MYSQL_ARGS="-DMYSQL_DYNAMIC_PLUGIN -DMYSQL_ABI_CHECK -fpic"
# LIBS="-lpthread -ldl"
# INCLUDES="-I /usr/include/mysql"
# gcc $MYSQL_ARGS -shared -o libauth_simple.so \
# $INCLUDES \
# src/cpp/plugin.c \
# target/debug/librust_mysql_plugin.a \
# $LIBS

FROM mariadb:11

COPY --from=crustdb /libcrustdb.so /usr/lib/mysql/plugin/crustdb.so