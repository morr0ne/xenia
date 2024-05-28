use linux_raw_sys::general::{
    __NR_chdir, __NR_chroot, __NR_close, __NR_execve, __NR_exit_group, __NR_getpid, __NR_mkdirat,
    __NR_mount, __NR_newfstatat, __NR_uname, __NR_write,
};

#[repr(transparent)]
pub struct Syscall(usize);

impl Syscall {
    pub const CHDIR: Self = Self::from_raw(__NR_chdir);
    pub const WRITE: Self = Self::from_raw(__NR_write);
    pub const CHROOT: Self = Self::from_raw(__NR_chroot);
    pub const CLOSE: Self = Self::from_raw(__NR_close);
    pub const EXECVE: Self = Self::from_raw(__NR_execve);
    pub const UNAME: Self = Self::from_raw(__NR_uname);
    pub const EXIT_GROUP: Self = Self::from_raw(__NR_exit_group);
    pub const GETPID: Self = Self::from_raw(__NR_getpid);
    pub const MKDIRAT: Self = Self::from_raw(__NR_mkdirat);
    pub const NEWFSTATAT: Self = Self::from_raw(__NR_newfstatat);
    pub const MOUNT: Self = Self::from_raw(__NR_mount);

    const fn from_raw(nr: u32) -> Self {
        Self(nr as usize)
    }

    #[inline]
    pub(crate) const fn as_raw(self) -> usize {
        self.0
    }
}
