use core::{ffi::CStr, mem::MaybeUninit};

use linux_raw_sys::{general::__NR_uname, system::new_utsname};
use xenia::syscall::syscall1;

fn main() {
    let mut uname = MaybeUninit::<new_utsname>::uninit();

    unsafe {
        let _ret = syscall1(__NR_uname as usize, uname.as_mut_ptr() as usize);
    }

    let uname = unsafe { uname.assume_init() };

    let nodename = unsafe { CStr::from_ptr(uname.release.as_ptr()) };

    dbg!(nodename);
}
