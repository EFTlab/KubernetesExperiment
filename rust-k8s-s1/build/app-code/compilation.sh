#!/bin/bash

# Move the code from the volume
echo Copying
cp -rf /app-code/* /app
# Compile the code
echo Compiling
#CARGO_HTTP_DEBUG=true CARGO_LOG=cargo::ops::registry=debug RUST_LOG=debug cargo build --release --verbose
cargo build --release
echo Moving
# Move the resulting binary to the output volume
mv /app/target/release/http_server_s1 /binary
