FROM rust:1.43.1-alpine3.11 AS builder

WORKDIR /tmp/build

ADD . ./

RUN cargo build --release

FROM alpine:3.11

COPY --from=builder \
  /tmp/build