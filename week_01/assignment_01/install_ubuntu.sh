#!/usr/bin/env bash

echo "Installing Rust in Ubuntu..."
wget -O rust.sh https://sh.rustup.rs
chmod +x rust.sh
./rust.sh -y
rm rust.sh
rustup --version

echo "Installing Cargo"
source $HOME/.cargo/env
cargo --version
