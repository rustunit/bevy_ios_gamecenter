# build-rust.sh

#!/bin/bash

set -e

THISDIR=$(dirname $0)
cd $THISDIR

touch ./src/lib.rs
cargo build --release --target aarch64-apple-ios
cargo build --release --target aarch64-apple-ios-sim
cargo build --release --target x86_64-apple-ios
mkdir -p ./target/universal-ios/debug

lipo \
    ./target/aarch64-apple-ios-sim/debug/libbevy_ios_gamecenter.a \
    ./target/x86_64-apple-ios/debug/libbevy_ios_gamecenter.a -create -output \
    ./target/universal-ios/debug/libbevy_ios_gamecenter.a