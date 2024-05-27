use bitflags::bitflags;
use core::{
    ffi::{c_char, c_uint, CStr},
    ptr::null_mut,
};
use linux_raw_sys::general::__NR_mount;

use crate::{syscall5_readonly, Result};

bitflags! {
    pub struct MountFlags: c_uint {
        const MOVE = linux_raw_sys::general::MS_MOVE;
        const RDONLY = linux_raw_sys::general::MS_RDONLY;

        const _ = !0;
    }
}

pub fn mount(
    source: Option<&CStr>,
    target: &CStr,
    file_system_type: Option<&CStr>,
    flags: MountFlags,
    data: Option<&CStr>,
) -> Result<()> {
    // FIXME
    unsafe {
        syscall5_readonly(
            __NR_mount as usize,
            option_as_ptr(source) as usize,
            target.as_ptr() as usize,
            option_as_ptr(file_system_type) as usize,
            flags.bits() as usize,
            option_as_ptr(data) as usize,
        );
    }

    Ok(())
}

#[inline]
fn option_as_ptr(option: Option<&CStr>) -> *const c_char {
    match option {
        Some(s) => s.as_ptr(),
        None => null_mut(),
    }
}
