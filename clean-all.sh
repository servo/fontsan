#!/bin/sh

set -e

(cd ots; git clean -Xf)
(cd ots/third_party/brotli; git clean -Xf)

rm -rf Cargo.lock target/
cargo clean
