#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
#[path = "../libclamav/mbox.rs"]
pub mod mbox;
#[path = "../libclamav/message.rs"]
pub mod message;
