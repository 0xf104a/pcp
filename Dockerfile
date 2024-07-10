FROM alpine:latest AS builder
LABEL authors="Anna-Sophie Zaitsewa"

RUN apk add rust cargo

RUN mkdir /tmp/pcp
WORKDIR /tmp/pcp
COPY . .
RUN mkdir .cargo

RUN cargo build --release
RUN cp /tmp/pcp/target/release/pretty-copy /usr/bin/pcp

FROM alpine:latest
LABEL authors="Anna-Sophie Zaitsewa"

RUN apk add musl libgcc
COPY --from=builder /usr/bin/pcp /usr/bin/pcp

