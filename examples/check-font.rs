// Copyright 2015 The Servo Project Developers.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the LICENSE file.

#![feature(exit_status)]

extern crate fontsan;

use std::{env, io};
use std::io::prelude::*;

fn main() {
    let mut input = vec![];
    io::stdin().read_to_end(&mut input).unwrap();

    match fontsan::process(&input) {
        Ok(output) => println!("ok, result is {} bytes", output.len()),
        Err(_) => {
            println!("there's something fishy about this font!");
            env::set_exit_status(1);
        }
    }
}
