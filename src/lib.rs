// Copyright 2015 The Servo Project Developers.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the LICENSE file.

extern crate libc;

use std::{io, convert, fmt};
use libc::size_t;

#[cfg(not(test))]
#[link(name="ots_glue", kind="static")]
extern "C" { }

/// Errors that can occur when sanitising a font.
#[derive(Debug)]
pub enum Error {
    /// The font data is invalid and cannot safely be sanitised.
    ///
    /// Some data may have been written to the output stream!
    InvalidFont,

    /// An error occurred while writing the sanitised output.
    IoError(io::Error),
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl convert::From<io::Error> for Error {
    #[inline]
    fn from(e: io::Error) -> Error {
        Error::IoError(e)
    }
}

/// Sanitise a font file, writing the result to `output`.
#[inline]
pub fn process_and_write<W>(output: &mut W, font_data: &[u8]) -> Result<(), Error>
    where W: io::Write + io::Seek,
{
    let mut stream = ffi::RustOTSStream {
        wr: output,
        error: None,
    };
    unsafe {
        if 0 == ffi::RustOTS_Process(&mut stream, font_data.as_ptr(),
                                     font_data.len() as size_t) {
            return Err(Error::InvalidFont);
        }
    }
    match stream.error.take() {
        Some(e) => Err(Error::IoError(e)),
        None => Ok(()),
    }
}

/// Convenience wrapper for `process_and_write` which writes the result to memory.
#[inline]
pub fn process(font_data: &[u8]) -> Result<Vec<u8>, Error> {
    let mut out = io::Cursor::new(vec![]);
    try!(process_and_write(&mut out, font_data));
    Ok(out.into_inner())
}

/// Implementation details.
///
/// This is only public so that the linker will keep it.
#[allow(non_snake_case)]
pub mod ffi;
