FROM rust:alpine3.15
ENTRYPOINT [ "/bin/bash" ]

COPY . /app
WORKDIR /app
RUN cargo build --release

CMD ["/app/target/release/hestia"]