#!/bin/sh

set -xe

# Fake up enough of a zlib install around the miniz build
# to please ots's configure script.
fake_zlib="$OUT_DIR/fake-zlib-install"
mkdir -p "$fake_zlib/include" "$fake_zlib/lib"
cp "$CARGO_MANIFEST_DIR/src/fake-zlib.h" "$fake_zlib/include/zlib.h"
cp "$DEP_MINIZ_ROOT/libminiz.a" "$fake_zlib/lib/libz.a"

cd ots/

# Can't run autogen.sh directly due to assumptions about Git layout
git submodule init
git submodule update
autoreconf --force --install --verbose

# Make sure we include the miniz-based header and not a system zlib.
FLAGS="-fPIC -include $OUT_DIR/fake-zlib-install/include/zlib.h"
./configure --with-zlib="$fake_zlib" \
    CFLAGS="$FLAGS" CXXFLAGS="$FLAGS" \
    LDFLAGS="-L $fake_zlib/lib"
make
