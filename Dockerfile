# Rust as the base image
FROM rust:1.60 as build

# Create workspace
WORKDIR /app

# Copy the source code
COPY . /app/

# Build for release.
RUN cargo build --release

# The final base image
FROM debian:buster-slim

# Copy from the previous build
COPY --from=build /hestia/target/release/hestia /usr/bin/hestia

# Run the binary
CMD ["/usr/src/hestia"]