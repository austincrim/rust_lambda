#!/bin/bash
cargo build --release --target x86_64-unknown-linux-musl
mkdir -p target/lambda && cp target/x86_64-unknown-linux-musl/release/bootstrap target/lambda