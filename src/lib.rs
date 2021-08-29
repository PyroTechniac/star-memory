#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]
#![allow(clippy::missing_errors_doc)]

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    time::Duration,
};

#[derive(Debug)]
pub struct VoidError;

impl Display for VoidError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("this shouldn't happen")
    }
}

impl Error for VoidError {}

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod win;

#[cfg(unix)]
pub fn memory() -> procfs::ProcResult<u64> {
    unix::get_memory()
}

#[cfg(unix)]
pub fn uptime() -> Result<Duration, unix::UnixError> {
    unix::get_uptime()
}

#[cfg(windows)]
pub fn memory() -> windows::Result<u64> {
    win::get_memory()
}

#[cfg(windows)]
pub fn uptime() -> Result<Duration, VoidError> {
    win::get_uptime()
}