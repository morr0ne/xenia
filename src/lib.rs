#![no_std]

pub mod syscall;

#[repr(transparent)]
pub struct Errno(u16);
