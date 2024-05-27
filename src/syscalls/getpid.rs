use linux_raw_sys::general::{__NR_getpid, __kernel_pid_t};

use crate::syscall0_readonly;

pub fn getpid() -> __kernel_pid_t {
    unsafe { syscall0_readonly(__NR_getpid as usize) as __kernel_pid_t }
}
