pub fn get() -> procfs::ProcResult<u64> {
    Ok(procfs::page_size()? as u64)
}
