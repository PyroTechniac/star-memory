use super::VoidError;
use std::time::Duration;
use win_bindings::Windows::System::Diagnostics::ProcessDiagnosticInfo;

pub fn get_memory() -> windows::Result<u64> {
    ProcessDiagnosticInfo::GetForCurrentProcess()?
        .MemoryUsage()?
        .GetReport()?
        .PageFileSizeInBytes()
}

pub fn get_uptime() -> Result<Duration, VoidError> {
    let ret = unsafe { kernel32::GetTickCount64() };
    Ok(Duration::from_millis(ret))
}
