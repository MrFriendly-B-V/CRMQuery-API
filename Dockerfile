FROM rust:latest as builder
COPY ./ /usr/local/src/

WORKDIR /usr/local/src
RUN cargo install --path .


FROM ubuntu:latest
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y \
    ca-certificates

COPY --from=builder /usr/local/cargo/bin/crm_query /usr/local/bin/crm_query
ENV USE_ENVIRONMENTAL_VARIABLES=Y

ENTRYPOINT ["crm_query"]