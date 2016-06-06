#!/bin/sh

set -xe

# Fake up enough of a zlib install around the miniz build
# to please ots's configure script.
fake_zlib="$OUT_DIR/fake-zlib-install"
mkdir -p "$fake_zlib/include" "$fake_zlib/lib"
cp "$CARGO_MANIFEST_DIR/src/fake-zlib.h" "$fake_zlib/include/zlib.h"
cp "$DEP_MINIZ_ROOT/libminiz.a" "$fake_zlib/lib/libz.a"

LDFLAGS="-L $fake_zlib/lib"

if [ "$TARGET" != "$HOST" ]; then
    CXX="$TARGET-g++"
    CC="$TARGET-gcc"
    AR="$TARGET-ar"
    if [ "${TARGET#*androideabi}" != "$TARGET" ]; then
        if [ -n "$DEP_FREETYPE_OUTDIR" ]; then
            LDFLAGS="$LDFLAGS -L $DEP_FREETYPE_OUTDIR"
        fi
        LDFLAGS="$LDFLAGS -Wl,--sysroot=$ANDROID_TOOLCHAIN/sysroot"
    fi
else
    CXX=g++
    CC=gcc
    AR=ar
fi

cd ots/

# Can't run autogen.sh directly due to assumptions about Git layout
git submodule update --init --recursive

if which autoreconf >/dev/null; then
    autoreconf --force --install --verbose
else
    echo
    echo 'WARNING: Using a snapshot of autoconf output for ots.'
    echo
    cp "$CARGO_MANIFEST_DIR/scripts/ots-autoconf-snapshot"/* .
fi

# Make sure we include the miniz-based header and not a system zlib.
FLAGS="-fPIC -include $OUT_DIR/fake-zlib-install/include/zlib.h"
if [ "$DEBUG" = true ]; then
    FLAGS="$FLAGS -g"
else
    FLAGS="$FLAGS -O2"
fi

if [ -n "$DEP_FREETYPE_OUTDIR" ]; then
    FREETYPE_CFLAGS="-I $DEP_FREETYPE_OUTDIR/include"
else
    FREETYPE_CFLAGS=`pkg-config --cflags freetype2`
fi

SRC_DIR="$PWD"

cd $OUT_DIR
$SRC_DIR/configure --with-zlib="$fake_zlib" --host="$TARGET" \
    CFLAGS="$FLAGS" CXXFLAGS="$FLAGS" \
    LDFLAGS="$LDFLAGS" \
    FREETYPE_CFLAGS="$FREETYPE_CFLAGS"

make
