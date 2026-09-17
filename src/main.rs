pub mod shared;
pub mod os;
pub mod daemon;
pub mod tray;

use os::ActiveProvider;
use shared::DaemonCommand;
use std::sync::mpsc;
use std::thread;

// Isolate Windows-only imports
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, TranslateMessage, MSG,
};

fn main() {
    let (tx, rx) = mpsc::channel::<DaemonCommand>();

    // 2. Instantiate the hardware/notification provider dynamically based on the OS
    let provider = ActiveProvider::new();

    let daemon_handle = thread::spawn(move || {
        daemon::poller::run_loop(provider, rx);
    });

    let tray_manager = tray::TrayManager::new();

    // 5a. Native Win32 Message Pump (Windows Only)
    #[cfg(target_os = "windows")]
    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);

            if tray_manager.handle_events(&tx) {
                break; 
            }
        }
    }

    // 5b. Standard Polling Loop (macOS / Linux)
    #[cfg(not(target_os = "windows"))]
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
        if tray_manager.handle_events(&tx) {
            break;
        }
    }

    daemon_handle.join().unwrap();
    println!("Application shut down cleanly.");
}