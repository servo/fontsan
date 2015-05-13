// Copyright 2015 The Servo Project Developers.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the LICENSE file.

extern crate fontsan;

static FIRA_SANS: &'static [u8]
    = include_bytes!("data/FiraSans-Regular.ttf");

static NOT_A_FONT: &'static [u8] = br#"
      ___           ___           ___                         ___
     /\__\         /\__\         /\  \          ___          /\  \
    /:/ _/_       /:/ _/_       /::\  \        /\  \        /::\  \
   /:/ /\  \     /:/ /\__\     /:/\:\__\       \:\  \      /:/\:\  \
  /:/ /::\  \   /:/ /:/ _/_   /:/ /:/  /        \:\  \    /:/  \:\  \
 /:/_/:/\:\__\ /:/_/:/ /\__\ /:/_/:/__/___  ___  \:\__\  /:/__/ \:\__\
 \:\/:/ /:/  / \:\/:/ /:/  / \:\/:::::/  / /\  \ |:|  |  \:\  \ /:/  /
  \::/ /:/  /   \::/_/:/  /   \::/~~/~~~~  \:\  \|:|  |   \:\  /:/  /
   \/_/:/  /     \:\/:/  /     \:\~~\       \:\__|:|__|    \:\/:/  /
     /:/  /       \::/  /       \:\__\       \::::/__/      \::/  /
     \/__/         \/__/         \/__/        ~~~~           \/__/
"#;

#[test]
fn font_ok() {
    assert!(fontsan::process(FIRA_SANS).is_ok());
}

#[test]
fn reject_nonsense() {
    assert!(fontsan::process(NOT_A_FONT).is_err());
}
