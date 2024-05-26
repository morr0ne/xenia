use core::arch::asm;

mod exit_group;
mod uname;

pub use exit_group::*;
pub use uname::*;

#[inline]
pub unsafe fn syscall0_readonly(nr: usize) -> usize {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly),
    );
    ret
}

#[inline]
pub unsafe fn syscall1(mut nr: usize, arg0: usize) -> usize {
    asm!(
        "syscall",
        inlateout("rax") nr,
        in("rdi") arg0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags),
    );
    nr
}

#[inline]
pub unsafe fn syscall1_readonly(nr: usize, arg0: usize) -> usize {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") arg0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

#[inline]
pub unsafe fn syscall1_noreturn(nr: usize, arg0: usize) -> ! {
    asm!(
        "syscall",
        in("rax") nr,
        in("rdi") arg0,
        options(nostack, noreturn)
    )
}

#[inline]
pub unsafe fn syscall2(nr: usize, arg0: usize, arg1: usize) -> usize {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall2_readonly(nr: usize, arg0: usize, arg1: usize) -> usize {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

#[inline]
pub unsafe fn syscall3(nr: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall3_readonly(nr: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

#[inline]
pub unsafe fn syscall4(nr: usize, arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall4_readonly(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> usize {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

#[inline]
pub unsafe fn syscall5(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> usize {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        in("r8") arg4,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall5_readonly(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> usize {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        in("r8") arg4,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

#[inline]
pub unsafe fn syscall6(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> usize {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        in("r8") arg4,
        in("r9") arg5,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall6_readonly(
    nr: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> usize {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        in("r8") arg4,
        in("r9") arg5,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    ret
}
