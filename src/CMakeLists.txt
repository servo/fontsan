project(ots)
cmake_minimum_required(VERSION 2.6)

# Needed to avoid error
set(DUMMY ${CMAKE_BUILD_TYPE})

set(OTS_SOURCES
  ots/src/cff.cc
  ots/src/cff.h
  ots/src/cff_type2_charstring.cc
  ots/src/cff_type2_charstring.h
  ots/src/cmap.cc
  ots/src/cmap.h
  ots/src/cvt.cc
  ots/src/cvt.h
  ots/src/fpgm.cc
  ots/src/fpgm.h
  ots/src/gasp.cc
  ots/src/gasp.h
  ots/src/gdef.cc
  ots/src/gdef.h
  ots/src/glyf.cc
  ots/src/glyf.h
  ots/src/gpos.cc
  ots/src/gpos.h
  ots/src/gsub.cc
  ots/src/gsub.h
  ots/src/hdmx.cc
  ots/src/hdmx.h
  ots/src/head.cc
  ots/src/head.h
  ots/src/hhea.cc
  ots/src/hhea.h
  ots/src/hmtx.cc
  ots/src/hmtx.h
  ots/src/kern.cc
  ots/src/kern.h
  ots/src/layout.cc
  ots/src/layout.h
  ots/src/loca.cc
  ots/src/loca.h
  ots/src/ltsh.cc
  ots/src/ltsh.h
  ots/src/math.cc
  ots/src/math_.h
  ots/src/maxp.cc
  ots/src/maxp.h
  ots/src/metrics.cc
  ots/src/metrics.h
  ots/src/name.cc
  ots/src/name.h
  ots/src/os2.cc
  ots/src/os2.h
  ots/src/ots.cc
  ots/src/ots.h
  ots/src/post.cc
  ots/src/post.h
  ots/src/prep.cc
  ots/src/prep.h
  ots/src/vdmx.cc
  ots/src/vdmx.h
  ots/src/vhea.cc
  ots/src/vhea.h
  ots/src/vmtx.cc
  ots/src/vmtx.h
  ots/src/vorg.cc
  ots/src/vorg.h
)

set(BROTLI_SOURCES
  ots/third_party/brotli/dec/bit_reader.c
  ots/third_party/brotli/dec/bit_reader.h
  ots/third_party/brotli/dec/context.h
  ots/third_party/brotli/dec/decode.c
  ots/third_party/brotli/dec/decode.h
  ots/third_party/brotli/dec/dictionary.c
  ots/third_party/brotli/dec/dictionary.h
  ots/third_party/brotli/dec/huffman.c
  ots/third_party/brotli/dec/huffman.h
  ots/third_party/brotli/dec/port.h
  ots/third_party/brotli/dec/prefix.h
  ots/third_party/brotli/dec/state.c
  ots/third_party/brotli/dec/state.h
  ots/third_party/brotli/dec/transform.h
  ots/third_party/brotli/dec/types.h
)

set(WOFF2_SOURCES
  ots/third_party/woff2/src/buffer.h
  ots/third_party/woff2/src/round.h
  ots/third_party/woff2/src/store_bytes.h
  ots/third_party/woff2/src/table_tags.cc
  ots/third_party/woff2/src/table_tags.h
  ots/third_party/woff2/src/variable_length.cc
  ots/third_party/woff2/src/variable_length.h
  ots/third_party/woff2/src/woff2_common.cc
  ots/third_party/woff2/src/woff2_common.h
  ots/third_party/woff2/src/woff2_dec.cc
  ots/third_party/woff2/src/woff2_dec.h
  ots/third_party/woff2/src/woff2_out.cc
  ots/third_party/woff2/src/woff2_out.h
)

set(OTS_GLUE_SOURCES
  ots_glue.cc
)

include_directories(
  fake-zlib
  ots/include
  ots/third_party/brotli/dec
  ots/third_party/woff2/src
)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O3 -fPIC")

add_library(ots STATIC ${OTS_SOURCES})
add_library(brotli STATIC ${BROTLI_SOURCES})
add_library(woff2 STATIC ${WOFF2_SOURCES})
add_library(ots_glue STATIC ${OTS_GLUE_SOURCES})

SET_TARGET_PROPERTIES(woff2 PROPERTIES
  COMPILE_FLAGS "-std=c++11"
)

install(TARGETS ots ARCHIVE DESTINATION lib)
install(TARGETS brotli ARCHIVE DESTINATION lib)
install(TARGETS woff2 ARCHIVE DESTINATION lib)
install(TARGETS ots_glue ARCHIVE DESTINATION lib)
