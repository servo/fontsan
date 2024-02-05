# fontsan

[![Build Status](https://travis-ci.org/servo/fontsan.svg?branch=master)](https://travis-ci.org/servo/fontsan)

A sanitiser for untrusted font files. Currently, this is just a wrapper around
[ots](https://github.com/khaledhosny/ots), which it builds a copy of.

To update:

* check the appropriate upstream tag/commit of ots and its dependencies
* edit the `_TAG` variables at the top of the `src/deps/update_deps.sh` script
* $ `bash src/deps/update_deps.sh`
* $ `git add src/deps`
* Potentially adjust src/CMakeLists.txt to accommodate new and removed files from the dependencies
* make `cargo build` and `cargo test` work
* update the crate version number and the `CHANGELOG.md`

## License of fontsan and dependencies

- fontsan: BSD 3-clause license
- ots: BSD 3-clause license
- lz4: BSD 2-Clause license
- brotli: MIT license
- woff2: MIT license