# FROM rust:alpine3.19 AS builder


# COPY . /usr/src/app/service
# WORKDIR /usr/src/app/service

# RUN mkdir -p ${CARGO_HOME:-$HOME/.cargo}

# RUN echo '[source.crates-io]' > ${CARGO_HOME:-$HOME/.cargo}/config

# RUN echo "replace-with = 'mirror'" >> ${CARGO_HOME:-$HOME/.cargo}/config
# RUN echo '[source.mirror]' >> ${CARGO_HOME:-$HOME/.cargo}/config
# RUN echo 'registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"' >> ${CARGO_HOME:-$HOME/.cargo}/config

# RUN export RUSTUP_DIST_SERVER="https://mirrors.ustc.edu.cn/rust-static"
# RUN export RUSTUP_UPDATE_ROOT="https://mirrors.ustc.edu.cn/rust-static/rustup"

# RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories

# RUN apk update
# RUN apk add --no-cache musl-dev
# # RUN apk add --no-cache musl-tools

# RUN update-ca-certificates

# RUN rustup target add x86_64-unknown-linux-musl

# RUN cargo build -p service --release

FROM selenium/standalone-chrome:latest

USER root

# WORKDIR /home/runner/work/webshot/webshot

# RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories

# RUN apk update

# RUN apk add chromium

COPY target/x86_64-unknown-linux-musl/release/service ./

EXPOSE 9000

ENTRYPOINT [ "./service" ]
