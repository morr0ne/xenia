use linux_raw_sys::general::__NR_write;

use crate::{fd::AsRawFd, syscall3_readonly, Result};

#[inline]
pub fn write<Fd: AsRawFd>(fd: Fd, buf: &[u8]) -> Result<usize> {
    let res = unsafe { syscall3_readonly(__NR_write as usize, fd, buf, buf.len()) };

    Ok(res)
}
