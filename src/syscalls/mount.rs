use bitflags::bitflags;
use core::ffi::{c_uint, CStr};

use crate::{syscall5_readonly, Result, Syscall};

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
            Syscall::MOUNT,
            source,
            target,
            file_system_type,
            flags.bits(),
            data,
        );
    }

    Ok(())
}
