ARG OS=alpine
ARG OS_VERSION=3.16
ARG RUST_VERSION=1.63

FROM rust:${RUST_VERSION}-${OS}${OS_VERSION} as builder
RUN apk update && apk add musl-dev openssl-dev
ADD backend /builder
WORKDIR /builder
ENV RUSTFLAGS=-Ctarget-feature=-crt-static
RUN cargo build --release
WORKDIR /app
RUN mv /builder/target/release/notebook /app/notebook

FROM ${OS}:${OS_VERSION}
COPY --from=builder /app /app
WORKDIR /app
ENTRYPOINT [ "/app/notebook" ]