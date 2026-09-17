use crate::shared::DaemonCommand;
use muda::{Menu, MenuItem};
use std::sync::mpsc::Sender;
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

pub struct TrayManager {
    // We must keep a reference to the tray icon alive; if dropped, the icon vanishes from the taskbar.
    _tray_icon: TrayIcon,
    pause_id: muda::Id,
    resume_id: muda::Id,
    quit_id: muda::Id,
}

impl TrayManager {
    pub fn new() -> Self {
        let menu = Menu::new();
        let pause_item = MenuItem::new("Pause Monitor", true, None);
        let resume_item = MenuItem::new("Resume Monitor", true, None);
        let quit_item = MenuItem::new("Quit", true, None);

        menu.append_items(&[&pause_item, &resume_item, &quit_item])
            .unwrap();

        // Programmatically generate a 16x16 solid blue icon (RGBA) so the app runs instantly
        let icon_bytes = vec![0, 120, 255, 255; 16 * 16 * 4]; 
        let icon = Icon::from_rgba(icon_bytes, 16, 16).unwrap();

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("Battery 80/20 Protector")
            .with_icon(icon)
            .build()
            .unwrap();

        Self {
            _tray_icon: tray_icon,
            pause_id: pause_item.id().clone(),
            resume_id: resume_item.id().clone(),
            quit_id: quit_item.id().clone(),
        }
    }

    /// Checks the menu event channel. Returns true if a Shutdown command was handled.
    pub fn handle_events(&self, tx: &Sender<DaemonCommand>) -> bool {
        if let Ok(event) = muda::MenuEvent::receiver().try_recv() {
            if event.id == self.pause_id {
                let _ = tx.send(DaemonCommand::Pause);
            } else if event.id == self.resume_id {
                let _ = tx.send(DaemonCommand::Resume);
            } else if event.id == self.quit_id {
                let _ = tx.send(DaemonCommand::Shutdown);
                return true; // Break the main thread's loop
            }
        }
        false
    }
}