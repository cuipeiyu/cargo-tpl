#!/usr/bin/sh

set -e

cargo build --release

cp -rf target/release/cargo-tpl ~/.cargo/bin/

set +e

strip ~/.cargo/bin/cargo-tpl

echo "\t\033[32mAll Done\033[0m"
