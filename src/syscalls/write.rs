use linux_raw_sys::general::__NR_write;

use crate::{fd::RawFd, syscall3_readonly, Result};

#[inline]
pub fn write(fd: RawFd, buf: &[u8]) -> Result<usize> {
    let res = unsafe {
        syscall3_readonly(
            __NR_write as usize,
            fd as usize,
            buf.as_ptr() as usize,
            buf.len(),
        )
    };

    Ok(res)
}
