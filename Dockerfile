# OpenSSL Builder
FROM ubuntu:focal as OPENSSL-BUILDER
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y \
    musl-tools \
    libssl-dev \
    build-essential \
    wget

WORKDIR /

RUN ln -s /usr/include/x86_64-linux-gnu/asm /usr/include/x86_64-linux-musl/asm \
    && ln -s /usr/include/asm-generic /usr/include/x86_64-linux-musl/asm-generic \
    && ln -s /usr/include/linux /usr/include/x86_64-linux-musl/linux

RUN mkdir /musl

RUN wget https://github.com/openssl/openssl/archive/OpenSSL_1_1_1f.tar.gz
RUN tar zxvf OpenSSL_1_1_1f.tar.gz

WORKDIR /openssl-OpenSSL_1_1_1f/

RUN CC="musl-gcc -fPIE -pie" ./Configure no-shared no-async --prefix=/musl --openssldir=/musl/ssl linux-x86_64
RUN make depend
RUN make -j$(nproc)
RUN make install

# Program builder
FROM rust:1.53.0-bullseye as BUILDER
RUN apt update && apt install -y \
    musl-tools \
    pkgconf

RUN rustup target add x86_64-unknown-linux-musl

COPY ./src /usr/src/crm_query/src/
COPY ./Cargo.toml /usr/src/crm_query/

COPY --from=OPENSSL-BUILDER /musl /musl

WORKDIR /usr/src/crm_query/

ENV PKG_CONFIG_ALLOW_CROSS=1
ENV OPENSSL_STATIC=true
ENV OPENSSL_DIR=/musl

ENV RUSTFLAGS='-C link-arg=-s'
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime image
FROM alpine:latest
RUN apk add --no-cache ca-certificates
COPY --from=BUILDER /usr/src/crm_query/target/x86_64-unknown-linux-musl/release/crm_query /usr/local/bin/crm_query
COPY ./log4rs.yaml /usr/local/bin/

EXPOSE 8080
WORKDIR /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/crm_query" ]