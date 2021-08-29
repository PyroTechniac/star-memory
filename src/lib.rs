#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]
#![allow(clippy::missing_errors_doc)]

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod win;

#[cfg(any(unix, doc))]
pub fn get() -> procfs::ProcResult<u64> {
    unix::get()
}

#[cfg(any(windows, doc))]
pub fn get() -> windows::Result<u64> {
    win::get()
}
