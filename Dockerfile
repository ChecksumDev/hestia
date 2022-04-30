FROM rust:alpine3.15
ENTRYPOINT [ "/bin/bash" ]

COPY . /app
RUN cargo build --release

CMD ["/app/target/release/hestia"]