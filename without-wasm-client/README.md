# HTTP client

## Prerequisites

- Podman
- Rust

## Step by step guide

Compile the Rust source code project:

```
$ rustup target add x86_64-unknown-linux-musl
$ cargo build --target x86_64-unknown-linux-musl --release
```

Run the client.

```
$ ./target/x86_64-unknown-linux-musl/release/client-without-wasm
Response: 200 OK
Body: b"{\n  \"args\": {}, \n  \"headers\": {\n    \"Host\": \"httpbin.org\", \n    \"X-Amzn-Trace-Id\": \"Root=1-67b86d6b-24defe4f3fa641f60440076d\"\n  }, \n  \"origin\": \"87.223.254.123\", \n  \"url\": \"http://httpbin.org/get\"\n}\n"
```

## Build and publish on Docker

The `Containerfile` follows the above steps to build and package a lightweight OCI-compliant container image.
```
$ podman build -t client-without-wasm .
... ...
```

Then we can run it:
```
$ podman run -dp 8080:8080 --rm client-without-wasm
Response: 200 OK
Body: b"{\n  \"args\": {}, \n  \"headers\": {\n    \"Host\": \"httpbin.org\", \n    \"X-Amzn-Trace-Id\": \"Root=1-67b86d6b-24defe4f3fa641f60440076d\"\n  }, \n  \"origin\": \"87.223.254.123\", \n  \"url\": \"http://httpbin.org/get\"\n}\n"
```

