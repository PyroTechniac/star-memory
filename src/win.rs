use win_bindings::Windows::System::Diagnostics::ProcessDiagnosticInfo;

pub fn get() -> windows::Result<u64> {
    ProcessDiagnosticInfo::GetForCurrentProcess()?
        .MemoryUsage()?
        .GetReport()?
        .PageFileSizeInBytes()
}
