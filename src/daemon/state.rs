use crate::shared::{BatteryStatus, NotifyState};

pub struct BatteryStateMachine {
    current_state: NotifyState,
}

impl BatteryStateMachine {
    pub fn new() -> Self {
        Self {
            current_state: NotifyState::Safe,
        }
    }

    /// Evaluates the current battery status against the thresholds and returns 
    /// an optional tuple of (Title, Message) if a notification needs to be fired.
    pub fn evaluate(&mut self, status: BatteryStatus) -> Option<(&'static str, &'static str)> {
        match self.current_state {
            NotifyState::Safe => {
                if status.level >= 80 && status.is_charging {
                    self.current_state = NotifyState::WarnedHigh;
                    return Some((
                        "Battery Charged up to 80%", 
                        "Unplug your charger to protect battery health."
                    ));
                } else if status.level <= 20 && !status.is_charging {
                    self.current_state = NotifyState::WarnedLow;
                    return Some((
                        "Battery Low down to 20%", 
                        "Plug in your charger to prevent shutdown."
                    ));
                }
            }
            NotifyState::WarnedHigh => {
                // Hysteresis reset: Reset to Safe only when battery drops safely below 75%
                // or if the user unplugs the charger.
                if status.level < 75 || !status.is_charging {
                    self.current_state = NotifyState::Safe;
                }
            }
            NotifyState::WarnedLow => {
                // Hysteresis reset: Reset to Safe only when battery charges back above 25%
                // or if the user plugs the charger back in.
                if status.level > 25 || status.is_charging {
                    self.current_state = NotifyState::Safe;
                }
            }
        }
        None
    }
}