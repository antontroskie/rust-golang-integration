#!/usr/bin/env sh

cd rust-main && sh clean.sh && cd .. &&
cd rust-bridge && sh clean.sh && cd .. &&
cd golang-lib && sh clean.sh && cd ..
