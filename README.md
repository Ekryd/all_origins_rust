# all_origins_rust
Web proxy implemented in Rust and hosted in Docker container

## What is it?

This is a Proxy, a hosted web service that accepts a URL and fetches the web content for that address. 
This kind of service is mostly used when you want to avoid CORS problems. The service is written in Rust
and hosted in a small Alpine Docker container.

## How do I run it?

### You need to create the certificate files

I have only some pointers around this in the ssl/README.md

### Then you can compile it in the following ways

* Compile it with Rust and run it locally, by executing the all_origins_rust command

```console
cargo build --release
./target/release/all_origins_rust
```

* Build the Docker container and run in your containerized environment

```console
docker build . -t ekryd/allorigins:6.0.0
docker run --rm -p 38724:38724 -p 38725:38725 --name allorigins5 ekryd/allorigins:6.0.0
```

### Use the service

You can test the service by using cURL:

> curl http://localhost:38724/get?url=https://www.google.com

> curl --insecure https://localhost:38725/get?url=https://www.google.com

Or just enter the following in your web-browser:

> https://localhost:38725/get?url=https://www.google.com

> http://127.0.0.1:38724/get?url=http://google.com

## Functionality

TODO

## Acknowledgements 

Heavily inspired by https://github.com/gnuns/allOrigins

## Other useful commands

Test the code
```console
cargo test
```

List all docker images with same name
```console
docker images ekryd/allorigins:6.0.0
```

Save docker image as tar file
```console
docker save -o allorigins6_docker.tar ekryd/allorigins:6.0.0
```

Start a command shell inside a running container
```console
docker exec -i -t allorigins6 /bin/sh
```
