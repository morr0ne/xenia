use core::ffi::c_int;

use linux_raw_sys::general::__NR_exit_group;

use crate::syscall1_noreturn;

pub fn exit_group(exit_code: c_int) -> ! {
    unsafe { syscall1_noreturn(__NR_exit_group as usize, exit_code as usize) }
}
