use crate::{fd::AsRawFd, syscall1_readonly, Syscall};

#[inline]
pub unsafe fn close<Fd: AsRawFd>(fd: Fd) {
    let _ret = syscall1_readonly(Syscall::CLOSE, fd);
}
