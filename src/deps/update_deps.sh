#!/usr/bin/env bash
# A convenience script to update our vendored dependencies of `ots`.
# Since some of the upstream repositories are quite large, we just vendor
# the files we need.
# When updating, simply update the `XYZ_TAG` variables at the top of the file
# with the new upstream tag of the upstream project.

set -eux

LZ4_TAG="v1.10.0"
WOFF2_TAG="v1.0.2"
# v1.0.9 fixes brotli CVE-2020-8927
BROTLI_TAG="v1.0.9"
OTS_TAG="v9.2.0"

# Directory this script and the vendored dependencies are located in.
DEPS_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Create a temporary directory to clone the upstream sources.
BASE_TEMP_DIR="$(mktemp -d)"

LZ4_GIT_DIR="${BASE_TEMP_DIR}/lz4"
LZ4_DEPS_DIR="${DEPS_DIR}/lz4"

echo "Fetching upstream LZ4 ${LZ4_TAG}"

git clone --depth 1 --branch "${LZ4_TAG}" https://github.com/lz4/lz4.git "${LZ4_GIT_DIR}"
# Delete the folder in case upstream deleted any files.
rm -rf "${LZ4_DEPS_DIR:-}"
mkdir -p "${LZ4_DEPS_DIR}/lib"
cp "${LZ4_GIT_DIR}/lib/lz4.c" \
    "${LZ4_GIT_DIR}/lib/lz4.h" \
    "${LZ4_GIT_DIR}/lib/LICENSE" \
    "${LZ4_DEPS_DIR}/lib"

WOFF2_GIT_DIR="${BASE_TEMP_DIR}/woff2"
WOFF2_DEPS_DIR="${DEPS_DIR}/woff2"

echo "Fetching upstream woff2 ${WOFF2_TAG}"

# From woff2 we just need the woff2dec and woff2common components.
git clone --depth 1 --branch "${WOFF2_TAG}" https://github.com/google/woff2.git "${WOFF2_GIT_DIR}"
# Delete the folder in case upstream deleted any files.
rm -rf "${WOFF2_DEPS_DIR:-}"
mkdir -p "${WOFF2_DEPS_DIR}/src"
mkdir -p "${WOFF2_DEPS_DIR}/include/woff2"
cp -r "${WOFF2_GIT_DIR}/include" "${WOFF2_DEPS_DIR}"


cp "${WOFF2_GIT_DIR}/src/table_tags.cc" \
    "${WOFF2_GIT_DIR}/src/variable_length.cc" \
    "${WOFF2_GIT_DIR}/src/woff2_common.cc" \
    "${WOFF2_GIT_DIR}/src/woff2_dec.cc" \
    "${WOFF2_GIT_DIR}/src/woff2_out.cc" \
    "${WOFF2_GIT_DIR}"/src/*.h  \
    "${WOFF2_DEPS_DIR}/src"

cp  "${WOFF2_GIT_DIR}/LICENSE" "${WOFF2_DEPS_DIR}/."

BROTLI_GIT_DIR="${BASE_TEMP_DIR}/brotli"
BROTLI_DEPS_DIR="${DEPS_DIR}/brotli"

echo "Fetching upstream brotli ${BROTLI_TAG}"

git clone --depth 1 --branch "${BROTLI_TAG}" https://github.com/google/brotli.git "${BROTLI_GIT_DIR}"
# Delete the folder in case upstream deleted any files.
rm -rf "${BROTLI_DEPS_DIR:-}"
mkdir -p "${BROTLI_DEPS_DIR}/c/common"
mkdir -p "${BROTLI_DEPS_DIR}/c/dec"
mkdir -p "${BROTLI_DEPS_DIR}/c/enc"
mkdir -p "${BROTLI_DEPS_DIR}/c/include/brotli"

cp -r "${BROTLI_GIT_DIR}/c/common" "${BROTLI_DEPS_DIR}/c"
cp -r "${BROTLI_GIT_DIR}/c/dec" "${BROTLI_DEPS_DIR}/c"
cp -r "${BROTLI_GIT_DIR}/c/enc" "${BROTLI_DEPS_DIR}/c"
cp -r "${BROTLI_GIT_DIR}/c/include" "${BROTLI_DEPS_DIR}/c"

cp "${BROTLI_GIT_DIR}/LICENSE" "${BROTLI_DEPS_DIR}/."

OTS_GIT_DIR="${BASE_TEMP_DIR}/ots"
OTS_DEPS_DIR="${DEPS_DIR}/ots"

echo "Fetching upstream ots ${OTS_TAG}"

git clone --depth 1 --branch "${OTS_TAG}" https://github.com/khaledhosny/ots.git "${OTS_GIT_DIR}"
# Delete the folder in case upstream deleted any files.
rm -rf "${OTS_DEPS_DIR:-}"
mkdir -p "${OTS_DEPS_DIR}"
cp -r "${OTS_GIT_DIR}/include" "${OTS_DEPS_DIR}"
cp -r "${OTS_GIT_DIR}/src" "${OTS_DEPS_DIR}"
cp "${OTS_GIT_DIR}/LICENSE" "${OTS_DEPS_DIR}/."

echo "Done - please run 'git status' to check if there are any modified or new files in src/deps."
