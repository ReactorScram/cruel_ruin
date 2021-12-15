#!/usr/bin/env bash

set -euo pipefail

pushd cruel_ruin_rs
cargo build --release
popd

mkdir -p build
pushd build
cmake ../cruel_ruin_cpp -DCMAKE_BUILD_TYPE=Release
make -j 4
popd
