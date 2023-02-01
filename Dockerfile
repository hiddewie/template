FROM rust:1 AS builder

RUN rustup target add x86_64-unknown-linux-musl
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
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3 as runtime

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /usr/bin
COPY --from=builder /template/target/x86_64-unknown-linux-musl/release/template /usr/local/bin/template
USER template:template

ENTRYPOINT ["template"]
CMD []
