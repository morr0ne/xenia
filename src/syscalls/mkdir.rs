use core::ffi::CStr;

use bitflags::bitflags;
use linux_raw_sys::general::{__kernel_mode_t, AT_FDCWD};

use crate::{Result, Syscall, syscall3_readonly, syscall_result_unit};

bitflags! {
    pub struct Mode: __kernel_mode_t {

        const _ = !0;
    }
}

pub fn mkdir(path: &CStr, mode: Mode) -> Result<()> {
    syscall_result_unit(unsafe { syscall3_readonly(Syscall::MKDIRAT, AT_FDCWD, path, mode.bits()) })
}
