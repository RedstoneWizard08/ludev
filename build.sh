#!/bin/bash

set -e

if [[ "$*" == *"-i"* ]]; then
    sudo apt-get update

    sudo apt-get -y install pkg-config \
        librust-openssl-dev libssl-dev \
        openssl librust-openssl-sys-dev

    curl -fsSL https://raw.githubusercontent.com/RedstoneWizard08/Scripts/main/rustup-system-install.sh | sudo bash
fi

if [[ "$*" == *"-r"* ]] || [[ "$*" == *"-i"* ]]; then
    rustup target add "$(uname -m)-unknown-linux-gnu"
fi

cargo build --release --all-features --target \
    "$(uname -m)-unknown-linux-gnu"

cp "target/$(uname -m)-unknown-linux-gnu/release/libgdlib_loader.so" "./gdlib-loader-$(dpkg --print-architecture).node"

if [[ "$*" == *"-d"* ]]; then
    cp "./gdlib-loader-$(dpkg --print-architecture).node" ./index.node
fi
