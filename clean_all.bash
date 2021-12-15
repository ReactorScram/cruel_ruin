#!/usr/bin/env bash

set -euo pipefail

pushd cruel_ruin_rs
cargo clean
popd

rm -rf build
