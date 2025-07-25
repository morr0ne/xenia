use core::{arch::asm, ffi::CStr, mem::MaybeUninit, ptr::null};

mod chdir;
mod chroot;
mod close;
mod execve;
mod exit_group;
mod getpid;
mod mkdir;
mod mount;
mod stat;
mod uname;
mod write;

pub use chdir::*;
pub use chroot::*;
pub use close::*;
pub use execve::*;
pub use exit_group::*;
pub use getpid::*;
pub use mkdir::*;
pub use mount::*;
pub use stat::*;
pub use uname::*;
pub use write::*;

use crate::{Errno, Result, Syscall, fd::AsFd};

pub unsafe trait SyscallArg: sealed::Sealed {
    fn as_arg(&self) -> usize;
}

unsafe impl<T> SyscallArg for &mut MaybeUninit<T> {
    #[inline]
    fn as_arg(&self) -> usize {
        self.as_ptr() as usize
    }
}

unsafe impl<T> SyscallArg for *mut T {
    #[inline]
    fn as_arg(&self) -> usize {
        *self as usize
    }
}

unsafe impl SyscallArg for &CStr {
    #[inline]
    fn as_arg(&self) -> usize {
        self.as_ptr() as usize
    }
}

unsafe impl<T: AsFd> SyscallArg for T {
    fn as_arg(&self) -> usize {
        self.as_fd().as_raw() as usize
    }
}

unsafe impl<T> SyscallArg for *const T {
    #[inline]
    fn as_arg(&self) -> usize {
        *self as usize
    }
}

unsafe impl SyscallArg for u32 {
    #[inline]
    fn as_arg(&self) -> usize {
        *self as usize
    }
}

unsafe impl SyscallArg for i32 {
    #[inline]
    fn as_arg(&self) -> usize {
        *self as usize
    }
}

unsafe impl SyscallArg for usize {
    #[inline]
    fn as_arg(&self) -> usize {
        *self
    }
}

unsafe impl SyscallArg for &[u8] {
    #[inline]
    fn as_arg(&self) -> usize {
        self.as_ptr() as usize
    }
}

unsafe impl<T: SyscallArg> SyscallArg for Option<T> {
    fn as_arg(&self) -> usize {
        match self {
            Some(a) => a.as_arg(),
            None => null::<T>() as usize,
        }
    }
}

mod sealed {
    use core::{ffi::CStr, mem::MaybeUninit};

    use crate::fd::AsFd;

    pub trait Sealed {}

    impl<T> Sealed for &mut MaybeUninit<T> {}
    impl Sealed for &CStr {}
    impl<T: AsFd> Sealed for T {}
    impl<T> Sealed for *const T {}
    impl<T> Sealed for *mut T {}
    impl Sealed for i8 {}
    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for isize {}
    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for usize {}
    impl Sealed for &[u8] {}
    impl Sealed for &mut [u8] {}
    impl<T: Sealed> Sealed for Option<T> {}
}

#[inline]
pub fn syscall_result(ret: usize) -> Result<usize> {
    if (-4095..0).contains(&(ret as isize)) {
        Err(unsafe { Errno::from_raw(ret as u16) })
    } else {
        Ok(ret)
    }
}

#[inline]
pub fn syscall_result_unit(ret: usize) -> Result<()> {
    if (-4095..0).contains(&(ret as isize)) {
        Err(unsafe { Errno::from_raw(ret as u16) })
    } else {
        Ok(())
    }
}

#[inline]
pub unsafe fn syscall0_readonly(syscall: Syscall) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly),
        );
        ret
    }
}

#[inline]
pub unsafe fn syscall1<Arg0: SyscallArg>(syscall: Syscall, arg0: Arg0) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags),
        );
        ret
    }
}

#[inline]
pub unsafe fn syscall1_readonly<Arg0: SyscallArg>(syscall: Syscall, arg0: Arg0) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly)
        );
        ret
    }
}

#[inline]
pub unsafe fn syscall1_noreturn<Arg0: SyscallArg>(syscall: Syscall, arg0: Arg0) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") syscall.as_raw(),
            in("rdi") arg0.as_arg(),
            options(nostack, noreturn)
        )
    }
}

#[inline]
pub unsafe fn syscall2<Arg0: SyscallArg, Arg1: SyscallArg>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
}

#[inline]
pub unsafe fn syscall2_readonly<Arg0: SyscallArg, Arg1: SyscallArg>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly)
        );
        ret
    }
}

#[inline]
pub unsafe fn syscall3<Arg0: SyscallArg, Arg1: SyscallArg, Arg2: SyscallArg>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
}

#[inline]
pub unsafe fn syscall3_readonly<Arg0: SyscallArg, Arg1: SyscallArg, Arg2: SyscallArg>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly)
        );
        ret
    }
}

#[inline]
pub unsafe fn syscall4<Arg0: SyscallArg, Arg1: SyscallArg, Arg2: SyscallArg, Arg3: SyscallArg>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            in("r10") arg3.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
}

#[inline]
pub unsafe fn syscall4_readonly<
    Arg0: SyscallArg,
    Arg1: SyscallArg,
    Arg2: SyscallArg,
    Arg3: SyscallArg,
>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            in("r10") arg3.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly)
        );
        ret
    }
}

#[inline]
pub unsafe fn syscall5<
    Arg0: SyscallArg,
    Arg1: SyscallArg,
    Arg2: SyscallArg,
    Arg3: SyscallArg,
    Arg4: SyscallArg,
>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            in("r10") arg3.as_arg(),
            in("r8") arg4.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
}

#[inline]
pub unsafe fn syscall5_readonly<
    Arg0: SyscallArg,
    Arg1: SyscallArg,
    Arg2: SyscallArg,
    Arg3: SyscallArg,
    Arg4: SyscallArg,
>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            in("r10") arg3.as_arg(),
            in("r8") arg4.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly)
        );
        ret
    }
}

#[inline]
pub unsafe fn syscall6<
    Arg0: SyscallArg,
    Arg1: SyscallArg,
    Arg2: SyscallArg,
    Arg3: SyscallArg,
    Arg4: SyscallArg,
    Arg5: SyscallArg,
>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            in("r10") arg3.as_arg(),
            in("r8") arg4.as_arg(),
            in("r9") arg5.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
}

#[inline]
pub unsafe fn syscall6_readonly<
    Arg0: SyscallArg,
    Arg1: SyscallArg,
    Arg2: SyscallArg,
    Arg3: SyscallArg,
    Arg4: SyscallArg,
    Arg5: SyscallArg,
>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            in("r10") arg3.as_arg(),
            in("r8") arg4.as_arg(),
            in("r9") arg5.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly)
        );
        ret
    }
}
