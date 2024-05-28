use core::{ffi::CStr, mem::MaybeUninit};

use linux_raw_sys::{general::__NR_uname, system::new_utsname};

use crate::syscall1;

#[repr(transparent)]
pub struct Uname {
    raw: new_utsname,
}

impl Uname {
    pub const unsafe fn from_raw(raw: new_utsname) -> Self {
        Self { raw }
    }

    pub fn sysname(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.raw.sysname.as_ptr()) }
    }

    pub fn nodename(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.raw.nodename.as_ptr()) }
    }

    pub fn release(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.raw.release.as_ptr()) }
    }

    pub fn version(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.raw.version.as_ptr()) }
    }

    pub fn machine(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.raw.machine.as_ptr()) }
    }

    pub fn domainname(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.raw.domainname.as_ptr()) }
    }
}

#[inline]
pub fn uname() -> Uname {
    let uname = MaybeUninit::<new_utsname>::uninit();

    unsafe {
        syscall1(__NR_uname as usize, uname);
        Uname::from_raw(uname.assume_init())
    }
}
