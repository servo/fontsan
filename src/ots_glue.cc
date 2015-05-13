// Copyright 2015 The Servo Project Developers.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the LICENSE file.

#include <stdint.h>

#include "opentype-sanitiser.h"

extern "C" {
    int RustOTSStream_WriteRaw(void *stream, const void *data, size_t length);
    int RustOTSStream_Seek(void *stream, off_t position);
    off_t RustOTSStream_Tell(void *stream);
}

class RustOTSStream : public ots::OTSStream {
public:
    RustOTSStream(void *stream) : stream(stream) { }

    virtual bool WriteRaw(const void *data, size_t length) {
        return RustOTSStream_WriteRaw(stream, data, length);
    }

    virtual bool Seek(off_t position) {
        return RustOTSStream_Seek(stream, position);
    }

    virtual off_t Tell() const {
        return RustOTSStream_Tell(stream);
    }

private:
    void *stream; // pointer to Rust object
};

extern "C" int RustOTS_Process(void *dest, const uint8_t *data, size_t length) {
    RustOTSStream stream(dest);
    ots::OTSContext context;

    return context.Process(&stream, data, length);
}
