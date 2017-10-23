# fontsan

[![Build Status](https://travis-ci.org/servo/fontsan.svg?branch=master)](https://travis-ci.org/servo/fontsan)

A sanitiser for untrusted font files. Currently, this is just a wrapper around
[ots](https://github.com/khaledhosny/ots), which it builds a copy of.

License: BSD 3-clause (same as ots)

To update:

* checkout the appropriate upstream commit in the src/ots submodule
* $ `cd src/ots`
* $ `git submodule update --init --recursive
* adjust src/CMakeLists.txt to accommodate new and removed files from the submodules
* make `cargo build` work
* update the crate version number