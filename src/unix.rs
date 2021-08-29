use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    mem,
    time::Duration,
};

#[derive(Debug)]
pub struct UnixError {
    kind: UnixErrorType,
}

impl UnixError {
    pub const fn kind(&self) -> UnixErrorType {
        self.kind
    }

    const fn info() -> Self {
        Self {
            kind: UnixErrorType::SysInfo,
        }
    }

    const fn ctl() -> Self {
        Self {
            kind: UnixErrorType::SysCtl,
        }
    }
}

impl Display for UnixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            UnixErrorType::SysCtl => f.write_str("sysctl failed"),
            UnixErrorType::SysInfo => f.write_str("sysinfo failed"),
        }
    }
}

impl Error for UnixError {}

#[derive(Debug, Clone, Copy)]
pub enum UnixErrorType {
    SysInfo,
    SysCtl,
}

#[cfg(unix)]
pub fn get_memory() -> procfs::ProcResult<u64> {
    Ok(procfs::page_size()? as u64)
}

#[cfg(target_os = "linux")]
pub fn get_uptime() -> Result<Duration, UnixError> {
    let mut info: libc::sysinfo = unsafe { mem::zeroed() };
    let ret = unsafe { libc::sysinfo(&mut info) };
    if ret == 0 {
        Ok(Duration::from_secs(info.uptime as u64))
    } else {
        Err(UnixError::info())
    }
}

#[cfg(any(
    target_os = "macos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd"
))]
pub fn get_uptime() -> Result<Duration, UnixError> {
    use std::time::SystemTime;
    let mut request = [libc::CTL_KERN, libc::KERN_BOOTTIME];
    let mut boottime: libc::timeval = unsafe { mem::zeroed() };
    let mut size: libc::size_t = mem::size_of_val(&boottime) as libc::size_t;
    let ret = unsafe {
        libc::sysctl(
            &mut request[0],
            2,
            &mut boottime as *mut libc::timeval as *mut libc::c_void,
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    };

    if ret == 0 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d - Duration::new(boottime.tv_sec as u64, boottime.tv_usec as u32 * 1000))
            .map_err(|_| UnixError::ctl())
    } else {
        Err(UnixError::ctl())
    }
}
