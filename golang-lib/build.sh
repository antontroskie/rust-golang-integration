#!/usr/bin/env sh

if [ -f .env ]; then
    export $(echo $(cat .env | sed 's/#.*//g' | xargs) | envsubst)
fi

MODULE_NAME=$(go list -m)
GIT_COMMIT=$(git rev-parse HEAD)
GIT_DIRTY=$(git diff --quiet || echo "-dirty")
BUILD_DATE=$(date '+%Y-%m-%d-%H:%M:%S')

go build -ldflags "-linkmode=external -X '${MODULE_NAME}/config.LibraryName="${BIN_NAME}"' -X '${MODULE_NAME}/config.Version=${VERSION}' -X '${MODULE_NAME}/config.GitCommit=${GIT_COMMIT}${GIT_DIRTY}' -X '${MODULE_NAME}/config.BuildDate=${BUILD_DATE}'" -o bin/${BIN_NAME} -buildmode=c-archive -o ../rust-main/build/libglang.a ./main.go
