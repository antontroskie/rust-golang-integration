#!/usr/bin/env sh

if [ -f .env ]; then
    export $(echo $(cat .env | sed 's/#.*//g' | xargs) | envsubst)
fi

cargo build --release &&
    mkdir -p ../rust-main/build &&
    cp target/release/libbridge.a ../golang-lib &&
    cp target/bridge.h ../golang-lib/libbridge.h &&
    cp target/bridge.h ../rust-main/build/libbridge.h &&
    cp target/release/libbridge.a ../rust-main/build/libbridge.a
