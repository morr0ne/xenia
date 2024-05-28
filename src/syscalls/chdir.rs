use core::ffi::CStr;

use crate::{syscall1_readonly, Result, Syscall};

pub fn chdir(path: &CStr) -> Result<()> {
    // FIXME
    let _ret = unsafe { syscall1_readonly(Syscall::CHDIR, path) };

    Ok(())
}
