FROM rust:1.72-alpine3.18 AS BUILD_IMAGE

RUN apk update
RUN apk add pkgconfig libressl-dev musl-dev

WORKDIR /usr/src/all_origins_rust

COPY . .

# Run test, for safety
RUN cargo test

# Compile
RUN cargo install --path .

FROM alpine:3.18

#The certs are not needed when we accept any cert in get_page.rs
#RUN apk update
#RUN apk add ca-certificates
#RUN update-ca-certificates
#RUN rm -rf /var/cache/apk

COPY --from=BUILD_IMAGE /usr/local/cargo/bin/all_origins_rust /usr/local/bin/all_origins_rust

WORKDIR /usr/src/all_origins_rust

COPY ssl ./ssl

EXPOSE 38724 38725
CMD ["all_origins_rust"]
