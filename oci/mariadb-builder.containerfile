FROM ubuntu:latest as builder

RUN sed -i 's/^Types: deb$/Types: deb deb-src/' /etc/apt/sources.list.d/ubuntu.sources

RUN apt update -y
RUN apt install -y git build-essential bison libgnutls28-dev python3 pkg-config libaio1t64 python3-pip

RUN ln -s /usr/lib/x86_64-linux-gnu/libaio.so.1t64 /usr/lib/libaio.so.1

RUN apt build-dep -y mariadb-server

VOLUME [ "/code" ]
WORKDIR /code/mariadb
