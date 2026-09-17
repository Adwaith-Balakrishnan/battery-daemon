use crate::shared::{BatteryError, BatteryStatus};

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub type ActiveProvider = windows::WindowsProvider;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub type ActiveProvider = macos::MacOsProvider;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub type ActiveProvider = linux::LinuxProvider;

pub trait BatteryProvider {
    fn check_battery(&self) -> Result<BatteryStatus, BatteryError>;
    fn send_notification(&self, title: &str, message: &str) -> Result<(), BatteryError>;
}