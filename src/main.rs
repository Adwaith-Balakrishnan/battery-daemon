pub mod shared;
pub mod os;
pub mod daemon;
pub mod tray;

use os::windows::WindowsProvider;
use shared::DaemonCommand;
use std::sync::mpsc;
use std::thread;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, TranslateMessage, MSG,
};

fn main() {
    // 1. Establish the IPC communication channels
    let (tx, rx) = mpsc::channel::<DaemonCommand>();

    // 2. Instantiate our hardware/notification provider
    let provider = WindowsProvider::new();

    // 3. Spawn Thread A (The Engine) and pass ownership of the hardware provider and receiver channel
    let daemon_handle = thread::spawn(move || {
        daemon::poller::run_loop(provider, rx);
    });

    // 4. Initialize the Tray System on the Main Thread (Thread B)
    let tray_manager = tray::TrayManager::new();

    // 5. Run a native Win32 Message Pump. 
    // This blocks the main thread efficiently, waking up only when the user interacts with the tray.
    unsafe {
        let mut msg = MSG::default();
        // GetMessageW blocks until a window event happens (like clicking our tray menu)
        while GetMessageW(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);

            // Check if the user interacted with our specific context menu items
            if tray_manager.handle_events(&tx) {
                break; // Exit loop if 'Quit' was selected
            }
        }
    }

    // 6. Graceful Cleanup: Wait for the daemon thread to finish executing its shutdown tasks before closing
    daemon_handle.join().unwrap();
    println!("Application shut down cleanly.");
}