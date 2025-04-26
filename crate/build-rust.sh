# build-rust.sh

#!/bin/bash

set -e

THISDIR=$(dirname $0)
cd $THISDIR

touch ./src/lib.rs
cargo build --target aarch64-apple-ios
cargo build --target x86_64-apple-ios
# TODO: enable once https://github.com/rust-lang/rust-bindgen/issues/3181 is released
# cargo build --target aarch64-apple-ios-sim

mkdir -p ./target/universal-ios/debug

lipo -create \
    "./target/aarch64-apple-ios/debug/libbevy_ios_gamecenter.a" \
    "./target/x86_64-apple-ios/debug/libbevy_ios_gamecenter.a" \
    -output "./target/universal-ios/debug/libbevy_ios_gamecenter.a"
