// Copyright 2015 The Servo Project Developers.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the LICENSE file.

// Declares the zlib API subset used by ots, and links it to miniz.

#ifndef ZLIB_H
#define ZLIB_H

#ifdef __cplusplus
extern "C" {
#endif

// Adapted from miniz.c.

typedef unsigned char Bytef;
typedef unsigned long uLongf;

int mz_uncompress(unsigned char *pDest,
                  unsigned long *pDest_len,
                  const unsigned char *pSource,
                  unsigned long source_len);

#define uncompress mz_uncompress
#define inflateEnd mz_inflateEnd

#define Z_OK 0

#ifdef __cplusplus
}
#endif

#endif
