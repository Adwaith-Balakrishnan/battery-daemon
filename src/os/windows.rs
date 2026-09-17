use super::BatteryProvider;
use crate::shared::{BatteryError, BatteryStatus};
use windows::core::HSTRING;
use windows::Data::Xml::Dom::XmlDocument;
use windows::System::Power::{PowerManager, BatteryStatus as WinBatteryStatus};
use windows::UI::Notifications::{ToastNotification, ToastNotificationManager};

pub struct WindowsProvider;

impl WindowsProvider {
    pub fn new() -> Self {
        Self
    }
}

impl BatteryProvider for WindowsProvider {
    fn check_battery(&self) -> Result<BatteryStatus, BatteryError> {
        // Query the Windows PowerManager for the battery percentage
        let level = PowerManager::RemainingChargePercent()
            .map_err(|e| BatteryError::OsQueryFailed(e.to_string()))? as u8;

        // Query the charging state. We must rename it WinBatteryStatus to avoid colliding with our own struct.
        let win_status = PowerManager::BatteryStatus()
            .map_err(|e| BatteryError::OsQueryFailed(e.to_string()))?;
        
        let is_charging = win_status == WinBatteryStatus::Charging;

        Ok(BatteryStatus { level, is_charging })
    }

    fn send_notification(&self, title: &str, message: &str) -> Result<(), BatteryError> {
        // Windows requires desktop notifications to be constructed using an XML payload
        let toast_xml = XmlDocument::new()
            .map_err(|e| BatteryError::OsQueryFailed(e.to_string()))?;
            
        let xml_string = format!(
            "<toast><visual><binding template=\"ToastText02\"><text id=\"1\">{}</text><text id=\"2\">{}</text></binding></visual></toast>",
            title, message
        );
        
        toast_xml.LoadXml(&HSTRING::from(xml_string))
            .map_err(|e| BatteryError::OsQueryFailed(e.to_string()))?;

        let toast = ToastNotification::CreateToastNotification(&toast_xml)
            .map_err(|e| BatteryError::OsQueryFailed(e.to_string()))?;

        // To send a notification without a registered app installer, we temporarily borrow PowerShell's AppID.
        // In a full production release, you would register your own AppID during the .msi installation.
        let app_id = HSTRING::from("{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe");
        let notifier = ToastNotificationManager::CreateToastNotifierWithId(&app_id)
            .map_err(|e| BatteryError::OsQueryFailed(e.to_string()))?;

        notifier.Show(&toast)
            .map_err(|e| BatteryError::OsQueryFailed(e.to_string()))?;

        Ok(())
    }
}