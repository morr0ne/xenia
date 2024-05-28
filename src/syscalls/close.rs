use linux_raw_sys::general::__NR_close;

use crate::{fd::AsRawFd, syscall1_readonly};

#[inline]
pub unsafe fn close<Fd: AsRawFd>(fd: Fd) {
    let _ret = syscall1_readonly(__NR_close as usize, fd);
}
