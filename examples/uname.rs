use core::ffi::CStr;

use xenia::uname;

fn main() {
    let uname = uname();

    let release = unsafe { CStr::from_ptr(uname.release.as_ptr()) };

    dbg!(release);
}
