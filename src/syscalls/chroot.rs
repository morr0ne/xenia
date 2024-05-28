use core::ffi::CStr;

use crate::{syscall1_readonly, Result, Syscall};

pub fn chroot(path: &CStr) -> Result<()> {
    // FIXME
    unsafe {
        syscall1_readonly(Syscall::CHROOT, path);
    }
    Ok(())
}
