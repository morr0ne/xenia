use core::ffi::{c_char, CStr};

use crate::{syscall3_readonly, Errno, Syscall};

pub unsafe fn execve(
    path: &CStr,
    args: *const *const c_char,
    env_vars: *const *const c_char,
) -> Errno {
    let ret = unsafe { syscall3_readonly(Syscall::EXECVE, path, args, env_vars) };

    Errno::from_raw(ret as u16)
}
