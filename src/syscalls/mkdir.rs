use core::ffi::CStr;

use bitflags::bitflags;
use linux_raw_sys::general::{__NR_mkdirat, __kernel_mode_t, AT_FDCWD};

use crate::{syscall3_readonly, Result};

bitflags! {
    pub struct Mode: __kernel_mode_t {

        const _ = !0;
    }
}

pub fn mkdir(path: &CStr, mode: Mode) -> Result<()> {
    // FIXME
    unsafe {
        syscall3_readonly(
            __NR_mkdirat as usize,
            AT_FDCWD as usize,
            path.as_ptr() as usize,
            mode.bits() as usize,
        );
    }

    Ok(())
}
