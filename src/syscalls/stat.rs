use core::{ffi::CStr, mem::MaybeUninit};

use linux_raw_sys::general::{__NR_newfstatat, stat, AT_FDCWD};

use crate::{syscall4, Errno, Result};

pub fn stat(path: &CStr) -> Result<stat> {
    let stat = MaybeUninit::<stat>::uninit();

    // FIXME
    unsafe {
        let ret = syscall4(__NR_newfstatat as usize, AT_FDCWD, path, stat, 0usize);

        if ret != 0 {
            return Err(Errno::from_raw(ret as u16));
        }

        Ok(stat.assume_init())
    }
}
