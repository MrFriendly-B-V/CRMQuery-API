FROM docker-registry.k8s.array21.dev/rust-base:latest as BUILDER
COPY ./src /usr/src/crm_query/src/
COPY ./Cargo.toml /usr/src/crm_query/

WORKDIR /usr/src/crm_query/
ENV RUSTFLAGS='-C link-arg=-s'

RUN cargo +stable build --release --target x86_64-unknown-linux-musl

# Runtime image
FROM alpine:latest
RUN apk add --no-cache ca-certificates
COPY --from=BUILDER /usr/src/crm_query/target/x86_64-unknown-linux-musl/release/crm_query /usr/local/bin/crm_query

RUN chmod a+rx /usr/local/bin/*
RUN adduser crm_query -s /bin/false -D -H
USER crm_query

EXPOSE 8080
WORKDIR /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/crm_query" ]