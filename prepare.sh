#!/bin/bash
set -e

pushd prepare
cargo build --release
popd

./prepare/target/release/prepare "$@"
