use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use sysinfo::{get_current_pid, ProcessExt, RefreshKind, System, SystemExt};

#[derive(Debug)]
pub struct UnixError;

impl Display for UnixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("an error occurred getting memory usage")
    }
}

impl Error for UnixError {}

#[cfg(unix)]
#[allow(clippy::cast_sign_loss)]
pub fn get_memory() -> Result<u64, UnixError> {
    let refresh_kind = RefreshKind::new().with_memory();
    let system = System::new_with_specifics(refresh_kind);

    let current_process = system
        .process(get_current_pid().map_err(|_| UnixError)?)
        .ok_or(UnixError)?;

    Ok(current_process.memory() * 1024)
}
