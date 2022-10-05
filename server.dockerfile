ARG OS=alpine
ARG OS_VERSION=3.16
ARG RUST_VERSION=1.62

FROM rust:${RUST_VERSION}-${OS}${OS_VERSION} as builder
RUN apk update && apk add musl-dev openssl-dev
ADD server /builder
WORKDIR /builder
RUN cargo build --release
WORKDIR /app
RUN mv /builder/target/release/server /app/server

FROM ${OS}:${OS_VERSION}
COPY --from=builder /app /app
WORKDIR /app
ENTRYPOINT [ "/app/server" ]