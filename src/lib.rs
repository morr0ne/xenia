#![no_std]

use core::{ffi::c_int, mem::MaybeUninit};

use linux_raw_sys::{
    general::{__NR_exit_group, __NR_uname},
    system::new_utsname,
};
use syscall::{syscall1, syscall1_noreturn};

pub mod errno;
pub mod syscall;

pub fn uname() -> new_utsname {
    let mut uname = MaybeUninit::<new_utsname>::uninit();

    unsafe {
        syscall1(__NR_uname as usize, uname.as_mut_ptr() as usize);
        uname.assume_init()
    }
}

pub fn exit_group(exit_code: c_int) -> ! {
    unsafe { syscall1_noreturn(__NR_exit_group as usize, exit_code as usize) }
}
