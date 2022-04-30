# Rust as the base image
FROM rust:1.49 as build

# Create workspace
WORKDIR /app

# Copy our manifests
COPY ./Cargo.toml ./
COPY ./Cargo.lock ./

# Build only the dependencies to cache them
RUN cargo build --release \
    && rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/hestia* \
    && cargo build --release

# The final base image
FROM debian:buster-slim

# Copy from the previous build
COPY --from=build /hestia/target/release/hestia /usr/bin/hestia

# Run the binary
CMD ["/usr/src/hestia"]