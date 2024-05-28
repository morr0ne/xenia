use core::ffi::c_int;

use crate::{syscall1_noreturn, Syscall};

#[inline]
pub fn exit_group(exit_code: c_int) -> ! {
    unsafe { syscall1_noreturn(Syscall::EXIT_GROUP, exit_code) }
}
