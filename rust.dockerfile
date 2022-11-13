ARG OS=alpine
ARG OS_VERSION=3.16
ARG RUST_VERSION=1.65
ARG PKG

FROM rust:${RUST_VERSION}-${OS}${OS_VERSION} as builder
ARG PKG
RUN apk update && apk add musl-dev openssl-dev
ADD . /build
WORKDIR /build
# required by native tls
ENV RUSTFLAGS=-Ctarget-feature=-crt-static
RUN cargo build --release -p ${PKG}
WORKDIR /app
RUN mv /build/target/release/${PKG} /app/${PKG}

FROM ${OS}:${OS_VERSION}
ARG PKG
COPY --from=builder /app /app
# required by native tls
RUN apk update && apk add libgcc
WORKDIR /app
ENTRYPOINT [ "/app/noteboo$PKG" ]