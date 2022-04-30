FROM rust:alpine3.15
ENTRYPOINT [ "/bin/bash" ]

# trunk-ignore(hadolint/DL3018)
RUN apk add --no-cache --virtual .build-deps \
    ca-certificates \
    curl \
    gcc \
    git \
    glib \
    make \
    openssl \
    pcre \
    zlib \
    && rm -rf /var/cache/apk/*

COPY . /app
WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl \ 
    && cargo build --release

CMD ["/app/target/release/hestia"]