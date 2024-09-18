#!/usr/bin/env sh

if [ -f .env ]; then
    export $(echo $(cat .env | sed 's/#.*//g' | xargs) | envsubst)
fi

export OUT_DIR=./src
if [ "$(uname)" = "Darwin" ] && [ "$(uname -m)" = "arm64" ]; then
    RUSTFLAGS="-C link-arg=-framework -C link-arg=CoreFoundation -C link-arg=-framework -C link-arg=Security" cargo run --target aarch64-apple-darwin --bin main
else
    cargo run --bin main
fi
