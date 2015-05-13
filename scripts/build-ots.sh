#!/bin/sh

set -xe

cd ots/

# Can't run autogen.sh directly due to assumptions about Git layout

git submodule init
git submodule update
autoreconf --force --install --verbose
./configure CFLAGS=-fPIC CXXFLAGS=-fPIC
make
