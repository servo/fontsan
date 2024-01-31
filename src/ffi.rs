// Copyright 2015 The Servo Project Developers.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the LICENSE file.

use std::io::{self, Seek, SeekFrom, Write};
use std::slice;

use libc::{c_int, off_t, size_t};

pub trait Wr: Write + Seek {}
impl<T> Wr for T where T: Write + Seek {}

pub struct RustOTSStream<'a> {
    pub wr: &'a mut (dyn Wr + 'a),
    pub error: Option<io::Error>,
}

#[allow(improper_ctypes)]
extern "C" {
    pub fn RustOTS_Process(stream: *mut RustOTSStream, data: *const u8, len: size_t) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn RustOTSStream_WriteRaw(
    stream: *mut RustOTSStream,
    data: *const u8,
    len: size_t,
) -> c_int {
    let buf = slice::from_raw_parts(data, len as usize);
    // Return success/failure only!
    match (*stream).wr.write_all(buf) {
        Ok(()) => 1,
        Err(e) => {
            (*stream).error = Some(e);
            0
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn RustOTSStream_Seek(stream: *mut RustOTSStream, position: off_t) -> c_int {
    match (*stream).wr.seek(SeekFrom::Start(position as u64)) {
        Ok(_) => 1,
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn RustOTSStream_Tell(stream: *mut RustOTSStream) -> off_t {
    match (*stream).wr.seek(SeekFrom::Current(0)) {
        Err(e) => {
            // Should be impossible.
            // FIXME: bad idea to panic when called from C
            panic!("RustOTSStream_Tell: can't seek: {}", e);
        }
        Ok(p) => p as off_t,
    }
}
