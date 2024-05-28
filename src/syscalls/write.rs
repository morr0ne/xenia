use crate::{fd::AsRawFd, syscall3_readonly, Result, Syscall};

#[inline]
pub fn write<Fd: AsRawFd>(fd: Fd, buf: &[u8]) -> Result<usize> {
    let res = unsafe { syscall3_readonly(Syscall::WRITE, fd, buf, buf.len()) };

    Ok(res)
}
