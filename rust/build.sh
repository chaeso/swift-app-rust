#!/bin/sh

cargo lipo --release
cbindgen src/lib.rs -l c > rust.h

mkdir -p ../rust-ios-app/libs
mkdir -p ../rust-ios-app/include
cp rust.h ../rust-ios-app/include
cp target/universal/release/librust.a ../rust-ios-app/libs
