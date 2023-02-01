FROM rust:1 AS builder

RUN case "$apkArch" in \
        arm64) export RUST_TARGET="aarch64-unknown-linux-musl" ;; \
        amd64) export RUST_TARGET="x86_64-unknown-linux-musl" ;; \
    esac && \
    rustup target add "$RUST_TARGET"

RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

ENV USER=template
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /template
COPY ./ .
RUN cargo build --target "$RUST_TARGET" --release
RUN mv "/template/target/$RUST_TARGET/release/template" /usr/local/bin/template

FROM alpine:3 as runtime

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /usr/bin
COPY --from=builder /usr/local/bin/template /usr/local/bin/template
USER template:template

ENTRYPOINT ["template"]
CMD []
