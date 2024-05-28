use core::ffi::CStr;

use linux_raw_sys::general::__NR_chroot;

use crate::{syscall1_readonly, Result};

pub fn chroot(path: &CStr) -> Result<()> {
    // FIXME
    unsafe {
        syscall1_readonly(__NR_chroot as usize, path);
    }
    Ok(())
}
