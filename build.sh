#!/bin/bash

set -e

sudo apt-get update

sudo apt-get -y install pkg-config \
    librust-openssl-dev libssl-dev \
    openssl librust-openssl-sys-dev

rustup target add x86_64-unknown-linux-gnu \
    aarch64-unknown-linux-gnu arm-unknown-linux-gnueabi \
    arm-unknown-linux-gnueabihf

cross build --release --all-features --target \
    x86_64-unknown-linux-gnu

cross build --release --all-features --target \
    aarch64-unknown-linux-gnu

cross build --release --all-features --target \
    arm-unknown-linux-gnueabi

cross build --release --all-features --target \
    arm-unknown-linux-gnueabihf

cp target/x86_64-unknown-linux-gnu/release/libgdlib_loader.so ./gdlib-loader-amd64.node
cp target/aarch64-unknown-linux-gnu/release/libgdlib_loader.so ./gdlib-loader-arm64.node
cp target/arm-unknown-linux-gnueabi/release/libgdlib_loader.so ./gdlib-loader-arm.node
cp target/arm-unknown-linux-gnueabihf/release/libgdlib_loader.so ./gdlib-loader-armhf.node
