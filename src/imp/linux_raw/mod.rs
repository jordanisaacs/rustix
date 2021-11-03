mod arch;
mod conv;
mod reg;
mod vdso;
mod vdso_wrappers;

pub(crate) mod elf;
pub(crate) mod fs;
pub(crate) mod io;
pub(crate) mod net;
pub(crate) mod process;
pub(crate) mod rand;
pub(crate) mod syscalls;
pub(crate) mod thread;
pub(crate) mod time;

#[cfg(not(feature = "rustc-dep-of-std"))]
pub(crate) mod fd {
    pub(crate) use io_lifetimes::*;

    #[allow(unused_imports)]
    pub(crate) use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd, RawFd as LibcFd};
}

#[cfg(feature = "rustc-dep-of-std")]
pub(crate) use crate::io::fd;

// The linux_raw backend doesn't use actual libc, but it is convenient
// to have various types defined in a module called "libc", to keep the
// code consistent with the libc backend.
pub(crate) mod libc;
