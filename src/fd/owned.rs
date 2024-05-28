//! Owned and borrowed Unix-like file descriptors.

#![deny(unsafe_op_in_unsafe_fn)]

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;

use super::raw::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
// use crate::io::close;
use core::fmt;
use core::marker::PhantomData;
use core::mem::forget;

/// A borrowed file descriptor.
///
/// This has a lifetime parameter to tie it to the lifetime of something that owns the file
/// descriptor. For the duration of that lifetime, it is guaranteed that nobody will close the file
/// descriptor.
///
/// This uses `repr(transparent)` and has the representation of a host file
/// descriptor, so it can be used in FFI in places where a file descriptor is
/// passed as an argument, it is not captured or consumed, and it never has the
/// value `-1`.
///
/// This type's `.to_owned()` implementation returns another `BorrowedFd`
/// rather than an `OwnedFd`. It just makes a trivial copy of the raw file
/// descriptor, which is then borrowed under the same lifetime.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BorrowedFd<'fd> {
    fd: RawFd,
    _phantom: PhantomData<&'fd OwnedFd>,
}

/// An owned file descriptor.
///
/// This closes the file descriptor on drop. It is guaranteed that nobody else will close the file
/// descriptor.
///
/// This uses `repr(transparent)` and has the representation of a host file
/// descriptor, so it can be used in FFI in places where a file descriptor is
/// passed as a consumed argument or returned as an owned value, and it never
/// has the value `-1`.
#[repr(transparent)]
pub struct OwnedFd {
    fd: RawFd,
}

impl BorrowedFd<'_> {
    /// Return a `BorrowedFd` holding the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `fd` must remain open for the duration of
    /// the returned `BorrowedFd`, and it must not have the value `-1`.
    #[inline]
    pub const unsafe fn borrow_raw(fd: RawFd) -> Self {
        assert!(fd != u32::MAX as RawFd);
        // SAFETY: we just asserted that the value is in the valid range and isn't `-1` (the only value bigger than `0xFF_FF_FF_FE` unsigned)
        Self {
            fd,
            _phantom: PhantomData,
        }
    }
}

impl OwnedFd {
    // FIXME
    // /// Creates a new `OwnedFd` instance that shares the same underlying file
    // /// description as the existing `OwnedFd` instance.
    // pub fn try_clone(&self) -> crate::io::Result<Self> {
    //     self.as_fd().try_clone_to_owned()
    // }
}

impl BorrowedFd<'_> {
    // FIXME
    /// Creates a new `OwnedFd` instance that shares the same underlying file
    /// description as the existing `BorrowedFd` instance.
    // pub fn try_clone_to_owned(&self) -> crate::io::Result<OwnedFd> {
    //     // We want to atomically duplicate this file descriptor and set the
    //     // CLOEXEC flag, and currently that's done via F_DUPFD_CLOEXEC. This
    //     // is a POSIX flag that was added to Linux in 2.6.24.
    //     #[cfg(not(any(target_os = "espidf", target_os = "vita")))]
    //     let cmd = libc::F_DUPFD_CLOEXEC;

    //     // For ESP-IDF, F_DUPFD is used instead, because the CLOEXEC semantics
    //     // will never be supported, as this is a bare metal framework with
    //     // no capabilities for multi-process execution. While F_DUPFD is also
    //     // not supported yet, it might be (currently it returns ENOSYS).
    //     #[cfg(any(target_os = "espidf", target_os = "vita"))]
    //     let cmd = libc::F_DUPFD;

    //     // Avoid using file descriptors below 3 as they are used for stdio
    //     let fd = cvt(unsafe { libc::fcntl(self.as_raw_fd(), cmd, 3) })?;
    //     Ok(unsafe { OwnedFd::from_raw_fd(fd) })
    // }

    /// Creates a new `OwnedFd` instance that shares the same underlying file
    /// description as the existing `BorrowedFd` instance.
    #[cfg(any(target_arch = "wasm32", target_os = "hermit"))]
    #[stable(feature = "io_safety", since = "1.63.0")]
    pub fn try_clone_to_owned(&self) -> crate::io::Result<OwnedFd> {
        Err(crate::io::Error::UNSUPPORTED_PLATFORM)
    }
}

impl AsRawFd for BorrowedFd<'_> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl AsRawFd for OwnedFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl IntoRawFd for OwnedFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        let fd = self.fd;
        forget(self);
        fd
    }
}

impl FromRawFd for OwnedFd {
    /// Constructs a new instance of `Self` from the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `fd` must be open and suitable for assuming
    /// [ownership][io-safety]. The resource must not require any cleanup other than `close`.
    ///
    /// [io-safety]: io#io-safety
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        assert_ne!(fd, u32::MAX as RawFd);
        // SAFETY: we just asserted that the value is in the valid range and isn't `-1` (the only value bigger than `0xFF_FF_FF_FE` unsigned)
        Self { fd }
    }
}

impl Drop for OwnedFd {
    #[inline]
    fn drop(&mut self) {
        unsafe { crate::close(self.as_raw_fd()) }
    }
}

impl fmt::Debug for BorrowedFd<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BorrowedFd").field("fd", &self.fd).finish()
    }
}

impl fmt::Debug for OwnedFd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OwnedFd").field("fd", &self.fd).finish()
    }
}

/// A trait to borrow the file descriptor from an underlying object.
///
/// This is only available on unix platforms and must be imported in order to
/// call the method. Windows platforms have a corresponding `AsHandle` and
/// `AsSocket` set of traits.
pub trait AsFd {
    /// Borrows the file descriptor.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::fs::File;
    /// # use std::io;
    /// # #[cfg(any(unix, target_os = "wasi"))]
    /// # use std::os::fd::{AsFd, BorrowedFd};
    ///
    /// let mut f = File::open("foo.txt")?;
    /// # #[cfg(any(unix, target_os = "wasi"))]
    /// let borrowed_fd: BorrowedFd<'_> = f.as_fd();
    /// # Ok::<(), io::Error>(())
    /// ```
    fn as_fd(&self) -> BorrowedFd<'_>;
}

impl<T: AsFd + ?Sized> AsFd for &T {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        T::as_fd(self)
    }
}

impl<T: AsFd + ?Sized> AsFd for &mut T {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        T::as_fd(self)
    }
}

impl AsFd for BorrowedFd<'_> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        *self
    }
}

impl AsFd for OwnedFd {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        // Safety: `OwnedFd` and `BorrowedFd` have the same validity
        // invariants, and the `BorrowedFd` is bounded by the lifetime
        // of `&self`.
        unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
    }
}

/// This impl allows implementing traits that require `AsFd` on Arc.
/// ```
/// # #[cfg(any(unix, target_os = "wasi"))] mod group_cfg {
/// # #[cfg(target_os = "wasi")]
/// # use std::os::wasi::io::AsFd;
/// # #[cfg(unix)]
/// # use std::os::unix::io::AsFd;
/// use std::net::UdpSocket;
/// use std::sync::Arc;
///
/// trait MyTrait: AsFd {}
/// impl MyTrait for Arc<UdpSocket> {}
/// impl MyTrait for Box<UdpSocket> {}
/// # }
/// ```
impl<T: AsFd + ?Sized> AsFd for Arc<T> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        (**self).as_fd()
    }
}

impl<T: AsFd + ?Sized> AsFd for Rc<T> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        (**self).as_fd()
    }
}

impl<T: AsFd + ?Sized> AsFd for Box<T> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        (**self).as_fd()
    }
}
