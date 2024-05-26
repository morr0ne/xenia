use linux_raw_sys::general::__NR_close;

use crate::{fd::RawFd, syscall1_readonly};

pub unsafe fn close(fd: RawFd) {
    let _ret = syscall1_readonly(__NR_close as usize, fd as usize);
}
