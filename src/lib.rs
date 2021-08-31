#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]
#![allow(clippy::missing_errors_doc)]

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod win;

#[cfg(unix)]
pub fn memory() -> Result<u64, unix::UnixError> {
    unix::get_memory()
}

#[cfg(windows)]
pub fn memory() -> windows::Result<u64> {
    win::get_memory()
}
