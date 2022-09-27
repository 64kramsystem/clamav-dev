#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![feature(extern_types)]
#![feature(linkage)]
#![allow(clashing_extern_declarations)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod mbox;
pub mod message;
