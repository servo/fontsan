cmake_minimum_required(VERSION 3.13)
project(ots)

set(CMAKE_POSITION_INDEPENDENT_CODE ON)

file(GLOB OTS_SOURCES CONFIGURE_DEPENDS "deps/ots/src/*.cc")

file(GLOB_RECURSE BROTLI_SOURCES CONFIGURE_DEPENDS "deps/brotli/c/*.c")

set(WOFF2_SOURCES
  deps/woff2/src/table_tags.cc
  deps/woff2/src/variable_length.cc
  deps/woff2/src/woff2_common.cc
  deps/woff2/src/woff2_dec.cc
  deps/woff2/src/woff2_out.cc
)

set(OTS_GLUE_SOURCES
  ots_glue.cc
)

set(LZ4_SOURCES
    deps/lz4/lib/lz4.c
    deps/lz4/lib/lz4.h
)

include_directories(
  fake-zlib
  deps/ots/include
  ots/src
  deps/lz4/lib
  deps/brotli/c/dec
  deps/brotli/c/include
  deps/woff2/include
  deps/woff2/src
)


add_library(ots STATIC ${OTS_SOURCES} ${LZ4_SOURCES})
add_library(brotli STATIC ${BROTLI_SOURCES})
add_library(woff2 STATIC ${WOFF2_SOURCES})
add_library(ots_glue STATIC ${OTS_GLUE_SOURCES})

set_property(TARGET woff2 PROPERTY CXX_STANDARD 11)
set_property(TARGET ots PROPERTY CXX_STANDARD 11)
set_property(TARGET ots_glue PROPERTY CXX_STANDARD 11)


install(TARGETS ots ARCHIVE DESTINATION lib)
install(TARGETS brotli ARCHIVE DESTINATION lib)
install(TARGETS woff2 ARCHIVE DESTINATION lib)
install(TARGETS ots_glue ARCHIVE DESTINATION lib)
