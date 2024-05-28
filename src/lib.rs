#![no_std]

extern crate alloc;

mod errno;
mod nr;
mod syscalls;

pub mod fd;
pub mod stdio;

pub use errno::*;
pub use nr::Syscall;
pub use syscalls::*;
