#![allow(clippy::all)]
// Transpiled modules
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![feature(extern_types)]
#![feature(linkage)]
#![allow(clashing_extern_declarations)]

/*
 *  libclamav features written in Rust
 *
 *  Copyright (C) 2021-2022 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
 *
 *  Authors: Micah Snyder, Mickey Sola
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License version 2 as
 *  published by the Free Software Foundation.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 *  MA 02110-1301, USA.
 */

/// cbindgen:ignore
pub mod sys;

pub mod cdiff;
pub mod ffi_util;
pub mod fuzzy_hash;
pub mod logging;
pub mod util;

// Transpiled modules

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod mbox;
pub mod message;
