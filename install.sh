#!/usr/bin/env bash

if [ "$1" == "--remove" ]; then
    sudo rm -v /usr/local/bin/neko
    exit 0
fi

cargo build --release
sudo mv -v ./target/release/neko /usr/local/bin
cargo clean