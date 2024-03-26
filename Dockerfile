FROM rust:latest AS builder
LABEL authors="Anna-Sophie Zaitsewa"

RUN mkdir /tmp/pcp
WORKDIR /tmp/pcp
COPY . .

RUN cargo build --release
RUN cp /tmp/pcp/target/release/pretty-copy /usr/bin/pcp

FROM alpine:latest
LABEL authors="Anna-Sophie Zaitsewa"

COPY --from=builder /usr/bin/pcp /usr/bin/pcp

