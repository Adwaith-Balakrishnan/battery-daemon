use std::sync::mpsc::Receiver;
use std::time::Duration;
use crate::shared::DaemonCommand;
use crate::os::BatteryProvider;
use super::state::BatteryStateMachine;

pub fn run_loop<P: BatteryProvider>(provider: P, rx: Receiver<DaemonCommand>) {
    let mut state_machine = BatteryStateMachine::new();
    let mut is_paused = false;
    // We check every 60 seconds. Using a duration variable keeps it clean.
    let poll_interval = Duration::from_secs(60);

    loop {
        // Use recv_timeout to block the thread efficiently.
        // It wakes up instantly if a command is sent, or times out after 60 seconds to poll.
        match rx.recv_timeout(poll_interval) {
            Ok(DaemonCommand::Shutdown) => {
                break; // Exit the loop entirely, letting the thread join cleanly.
            }
            Ok(DaemonCommand::Pause) => {
                is_paused = true;
            }
            Ok(DaemonCommand::Resume) => {
                is_paused = false;
            }
            Err(_) => {
                // This represents a timeout event (60 seconds elapsed with no UI command)
                if !is_paused {
                    if let Ok(status) = provider.check_battery() {
                        if let Some((title, msg)) = state_machine.evaluate(status) {
                            let _ = provider.send_notification(title, msg);
                        }
                    }
                }
            }
        }
    }
}