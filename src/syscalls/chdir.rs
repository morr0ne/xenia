use core::ffi::CStr;

use linux_raw_sys::general::__NR_chdir;

use crate::{syscall1_readonly, Result};

pub fn chdir(path: &CStr) -> Result<()> {
    // FIXME
    let _ret = unsafe { syscall1_readonly(__NR_chdir as usize, path) };

    Ok(())
}
