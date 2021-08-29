fn main() {
    windows::build!(
        Windows::System::Diagnostics::{ProcessDiagnosticInfo, ProcessMemoryUsage, ProcessMemoryUsageReport},
    )
}
