# Create an initial image as build image
FROM rust:1.74-alpine3.18 AS BUILD_IMAGE

RUN apk update
RUN apk add pkgconfig libressl-dev musl-dev

WORKDIR /usr/src/all_origins_rust

# Copy needed files to the Docker image
COPY . .

# Run test, for safety
RUN cargo test

# Build the program for release
RUN cargo install --path .

# Build the release image ------
FROM alpine:3.18

# Get the compiled code from the build image
COPY --from=BUILD_IMAGE /usr/local/cargo/bin/all_origins_rust /usr/local/bin/all_origins_rust
WORKDIR /usr/src/all_origins_rust

# Add the certificates
COPY ssl ./ssl

# Expose the ports for http and https
EXPOSE 38724 38725

# Run the binary
CMD ["all_origins_rust"]
