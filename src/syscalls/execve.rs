use core::ffi::{c_char, CStr};

use linux_raw_sys::general::__NR_execve;

use crate::{syscall3_readonly, Errno};

pub unsafe fn execve(path: &CStr, args: *const *const c_char, env_vars: *const *const c_char) -> Errno {
    let ret = unsafe {
        syscall3_readonly(
            __NR_execve as usize,
            path.as_ptr() as usize,
            args as usize,
            env_vars as usize,
        )
    };

    Errno::from_raw(ret as u16)
}
