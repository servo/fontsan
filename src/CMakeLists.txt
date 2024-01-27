cmake_minimum_required(VERSION 3.13)
project(ots)

set(OTS_SOURCES
  ots/src/cff.cc
  ots/src/cff.h
  ots/src/cff_type2_charstring.cc
  ots/src/cff_type2_charstring.h
  ots/src/cmap.cc
  ots/src/cmap.h
  ots/src/cvt.cc
  ots/src/cvt.h
  ots/src/feat.cc
  ots/src/feat.h
  ots/src/fpgm.cc
  ots/src/fpgm.h
  ots/src/gasp.cc
  ots/src/gasp.h
  ots/src/gdef.cc
  ots/src/gdef.h
  ots/src/glat.cc
  ots/src/glat.h
  ots/src/gloc.cc
  ots/src/gloc.h
  ots/src/glyf.cc
  ots/src/glyf.h
  ots/src/gpos.cc
  ots/src/gpos.h
  ots/src/graphite.h
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
  ots/src/sile.cc
  ots/src/sile.h
  ots/src/silf.cc
  ots/src/silf.h
  ots/src/sill.cc
  ots/src/sill.h
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
  ots/third_party/brotli/c/common/dictionary.c
  ots/third_party/brotli/c/common/dictionary.h
  ots/third_party/brotli/c/dec/bit_reader.c
  ots/third_party/brotli/c/dec/bit_reader.h
  ots/third_party/brotli/c/dec/context.h
  ots/third_party/brotli/c/dec/decode.c
  ots/third_party/brotli/c/dec/huffman.c
  ots/third_party/brotli/c/dec/huffman.h
  ots/third_party/brotli/c/dec/port.h
  ots/third_party/brotli/c/dec/prefix.h
  ots/third_party/brotli/c/dec/state.c
  ots/third_party/brotli/c/dec/state.h
  ots/third_party/brotli/c/dec/transform.h
  ots/third_party/brotli/c/include/brotli/decode.h
  ots/third_party/brotli/c/include/brotli/port.h
  ots/third_party/brotli/c/include/brotli/types.h
  ots/third_party/brotli/c/enc/backward_references.c
  ots/third_party/brotli/c/enc/backward_references_hq.c
  ots/third_party/brotli/c/enc/bit_cost.c
  ots/third_party/brotli/c/enc/block_splitter.c
  ots/third_party/brotli/c/enc/brotli_bit_stream.c
  ots/third_party/brotli/c/enc/cluster.c
  ots/third_party/brotli/c/enc/compress_fragment.c
  ots/third_party/brotli/c/enc/compress_fragment_two_pass.c
  ots/third_party/brotli/c/enc/dictionary_hash.c
  ots/third_party/brotli/c/enc/encode.c
  ots/third_party/brotli/c/enc/entropy_encode.c
  ots/third_party/brotli/c/enc/histogram.c
  ots/third_party/brotli/c/enc/literal_cost.c
  ots/third_party/brotli/c/enc/memory.c
  ots/third_party/brotli/c/enc/metablock.c
  ots/third_party/brotli/c/enc/static_dict.c
  ots/third_party/brotli/c/enc/utf8_util.c
)

set(WOFF2_SOURCES
  ots/third_party/woff2/include/woff2/decode.h
  ots/third_party/woff2/include/woff2/encode.h
  ots/third_party/woff2/include/woff2/output.h
  ots/third_party/woff2/src/buffer.h
  ots/third_party/woff2/src/file.h
  ots/third_party/woff2/src/font.cc
  ots/third_party/woff2/src/font.h
  ots/third_party/woff2/src/glyph.cc
  ots/third_party/woff2/src/glyph.h
  ots/third_party/woff2/src/normalize.cc
  ots/third_party/woff2/src/normalize.h
  ots/third_party/woff2/src/port.h
  ots/third_party/woff2/src/round.h
  ots/third_party/woff2/src/store_bytes.h
  ots/third_party/woff2/src/table_tags.cc
  ots/third_party/woff2/src/table_tags.h
  ots/third_party/woff2/src/transform.cc
  ots/third_party/woff2/src/transform.h
  ots/third_party/woff2/src/variable_length.cc
  ots/third_party/woff2/src/variable_length.h
  ots/third_party/woff2/src/woff2_common.cc
  ots/third_party/woff2/src/woff2_common.h
  ots/third_party/woff2/src/woff2_dec.cc
  ots/third_party/woff2/src/woff2_enc.cc
  ots/third_party/woff2/src/woff2_out.cc
)

set(OTS_GLUE_SOURCES
  ots_glue.cc
)

set(LZ4_SOURCES
    ots/third_party/lz4/lib/lz4.c
    ots/third_party/lz4/lib/lz4.h
)

include_directories(
  fake-zlib
  ots/include
  ots/third_party/lz4/lib
  ots/third_party/brotli/c/dec
  ots/third_party/brotli/c/include
  ots/third_party/woff2/include
  ots/third_party/woff2/src
)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O3 -fPIC")

add_library(ots STATIC ${OTS_SOURCES})
add_library(brotli STATIC ${BROTLI_SOURCES})
add_library(woff2 STATIC ${WOFF2_SOURCES})
add_library(ots_glue STATIC ${OTS_GLUE_SOURCES})

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    target_sources(ots PRIVATE "${LZ4_SOURCES}")
    target_include_directories(ots PRIVATE ots/third_party/lz4/lib/)
endif()

SET_TARGET_PROPERTIES(woff2 PROPERTIES
  COMPILE_FLAGS "-std=c++11"
)

SET_TARGET_PROPERTIES(ots PROPERTIES
  COMPILE_FLAGS "-std=c++11"
)

install(TARGETS ots ARCHIVE DESTINATION lib)
install(TARGETS brotli ARCHIVE DESTINATION lib)
install(TARGETS woff2 ARCHIVE DESTINATION lib)
install(TARGETS ots_glue ARCHIVE DESTINATION lib)
