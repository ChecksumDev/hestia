FROM rust:alpine3.15
ENTRYPOINT [ "/bin/bash" ]

COPY . /app
WORKDIR /app
RUN cargo check --release \
    cargo build --release \
    cargo test --release

CMD ["/app/target/release/hestia"]