#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryStatus {
    pub level: u8,
    pub is_charging: bool,
}

#[derive(Debug)]
pub enum BatteryError {
    OsQueryFailed(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotifyState {
    Safe,
    WarnedHigh,
    WarnedLow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DaemonCommand {
    Pause,
    Resume,
    Shutdown,
}