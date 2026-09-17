use super::BatteryProvider;
use crate::shared::{BatteryError, BatteryStatus};
use std::fs;
use std::process::Command;

pub struct LinuxProvider;

impl LinuxProvider {
    pub fn new() -> Self { Self }
}

impl BatteryProvider for LinuxProvider {
    fn check_battery(&self) -> Result<BatteryStatus, BatteryError> {
        let capacity = fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
            .map_err(|e| BatteryError::OsQueryFailed(e.to_string()))?;
        let status = fs::read_to_string("/sys/class/power_supply/BAT0/status")
            .map_err(|e| BatteryError::OsQueryFailed(e.to_string()))?;

        let level = capacity.trim().parse::<u8>().unwrap_or(0);
        let is_charging = status.trim().eq_ignore_ascii_case("Charging");

        Ok(BatteryStatus { level, is_charging })
    }

    fn send_notification(&self, title: &str, message: &str) -> Result<(), BatteryError> {
        Command::new("notify-send").arg(title).arg(message).spawn()
            .map_err(|e| BatteryError::OsQueryFailed(e.to_string()))?;
        Ok(())
    }
}