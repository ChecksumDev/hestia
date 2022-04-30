FROM rust:alpine3.15
ENTRYPOINT [ "/bin/bash" ]

# trunk-ignore(hadolint/DL3018)
RUN apk add --no-cache \
    ca-certificates \
    curl \
    gcc \
    git \
    make \
    openssl \
    pcre \
    zlib \
    && rm -rf /var/cache/apk/*

COPY . /app
WORKDIR /app
RUN cargo build --release

CMD ["/app/target/release/hestia"]