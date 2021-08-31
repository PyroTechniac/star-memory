use std::time::Duration;
use win_bindings::Windows::System::Diagnostics::ProcessDiagnosticInfo;

pub fn get_memory() -> windows::Result<u64> {
    ProcessDiagnosticInfo::GetForCurrentProcess()?
        .MemoryUsage()?
        .GetReport()?
        .PageFileSizeInBytes()
}