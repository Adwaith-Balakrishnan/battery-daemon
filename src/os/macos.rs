use super::BatteryProvider;
use crate::shared::{BatteryError, BatteryStatus};
use std::process::Command;

pub struct MacOsProvider;

impl MacOsProvider {
    pub fn new() -> Self { Self }
}

impl BatteryProvider for MacOsProvider {
    fn check_battery(&self) -> Result<BatteryStatus, BatteryError> {
        let output = Command::new("pmset")
            .arg("-g").arg("batt")
            .output().map_err(|e| BatteryError::OsQueryFailed(e.to_string()))?;
            
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        let level = stdout.split('%').next()
            .and_then(|s| s.split_whitespace().last())
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(0);
            
        let is_charging = stdout.contains("charging") && !stdout.contains("discharging");
        Ok(BatteryStatus { level, is_charging })
    }

    fn send_notification(&self, title: &str, message: &str) -> Result<(), BatteryError> {
        let script = format!("display notification \"{}\" with title \"{}\"", message, title);
        Command::new("osascript").arg("-e").arg(&script).spawn()
            .map_err(|e| BatteryError::OsQueryFailed(e.to_string()))?;
        Ok(())
    }
}