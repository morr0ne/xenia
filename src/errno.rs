use linux_raw_sys::errno;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Errno {
    raw: u16,
}

impl Errno {
    pub const INTR: Self = Self::from_raw(errno::EINTR as u16);
    pub const NOENT: Self = Self::from_raw(errno::ENOENT as u16);

    pub const fn as_raw(&self) -> u16 {
        self.raw
    }

    pub const fn from_raw(raw: u16) -> Self {
        Self { raw }
    }
}

pub type Result<T, E = Errno> = core::result::Result<T, E>;
