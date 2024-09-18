#!/usr/bin/env sh

if [ -f .env ]; then
    export $(echo $(cat .env | sed 's/#.*//g' | xargs) | envsubst)
fi

# set -o allexport; source .env; set +o allexport

cd rust-bridge && sh build.sh && cd .. &&
    cd golang-lib && sh build.sh && cd .. &&
    cd rust-main && sh build.sh && cd ..

