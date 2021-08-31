#[cfg(unix)]
#[allow(clippy::cast_sign_loss)]
pub fn get_memory() -> procfs::ProcResult<u64> {
    Ok(procfs::page_size()? as u64)
}