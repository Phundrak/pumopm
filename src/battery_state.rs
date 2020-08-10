use notify_rust::{Hint, Notification, Urgency};

pub enum VerbosityLevel {
    None = 0,
    Some = 1,
    Lots = 2,
}

pub struct BatteryState {
    manager: battery::Manager,
    battery: battery::Battery,
    refresh_rate: u64,

    low_level: f32,
    very_low_level: f32,
    critical_level: f32,

    is_triggered_low: bool,
    is_triggered_very_low: bool,

    verbosity: VerbosityLevel,
}

pub const DEFAULT_LOW: f32 = 25_f32;
pub const DEFAULT_VERY_LOW: f32 = 15_f32;
pub const DEFAULT_CRITICAL: f32 = 10_f32;
pub const DEFAULT_REFRESH: u64 = 5;

impl BatteryState {
    pub fn new(
        low_level: f32,
        very_low_level: f32,
        critical_level: f32,
        refresh_rate: u64,
        verbosity: VerbosityLevel,
    ) -> battery::Result<Self> {
        let manager = battery::Manager::new().unwrap();
        let battery = match manager.batteries().unwrap().next() {
            Some(Ok(battery)) => battery,
            Some(Err(e)) => {
                eprintln!("An error occured: {}", e);
                return Err(e);
            }
            None => {
                eprintln!("Unable to find any battery");
                use std::io;
                return Err(io::Error::from(io::ErrorKind::NotFound).into());
            }
        };

        Ok(Self {
            manager,
            battery,
            refresh_rate,

            low_level,
            very_low_level,
            critical_level,

            is_triggered_low: false,
            is_triggered_very_low: false,

            verbosity,
        })
    }

    fn get_charge(&self) -> f32 {
        self.battery.state_of_charge().value * 100_f32
    }

    fn reset_levels(&mut self) {
        self.is_triggered_low = false;
        self.is_triggered_very_low = false;
    }

    fn trigger_low(&mut self) {
        if !self.is_triggered_low {
            self.is_triggered_low = true;
            let time = match self.battery.time_to_empty() {
                Some(e) => e.value.to_string(),
                None => String::from("unknown"),
            };
            let level = self.battery.state_of_charge().value * 100.0;
            let message =
                format!("Battery level is low! {} left ({}%)", time, level);
            Notification::new()
                .summary("Low Battery")
                .body(message.as_str())
                .hint(Hint::Category("battery".to_owned()))
                .urgency(Urgency::Normal)
                .show()
                .unwrap();
        }
    }

    fn trigger_very_low(&mut self) {
        if !self.is_triggered_very_low {
            self.is_triggered_very_low = true;
            let time = match self.battery.time_to_empty() {
                Some(e) => e.value.to_string(),
                None => String::from("unknown"),
            };
            let level = self.battery.state_of_charge().value * 100.0;
            let message = format!(
                "Battery level is very low! {} left ({}%)",
                time, level
            );
            Notification::new()
                .summary("Low Battery")
                .body(message.as_str())
                .hint(Hint::Category("battery".to_owned()))
                .urgency(Urgency::Critical)
                .show()
                .unwrap();
        }
    }

    pub fn update(&mut self) {
        self.manager.refresh(&mut self.battery).unwrap();
        use battery::State::{Discharging, Empty};
        match self.battery.state() {
            Discharging | Empty => match self.get_charge() {
                x if x < self.very_low_level => self.trigger_very_low(),
                x if x < self.low_level => self.trigger_low(),
                _ => {}
            },
            _ => self.reset_levels(),
        }

        use std::{thread, time::Duration};
        thread::sleep(Duration::from_secs(self.refresh_rate));
    }
}

impl Default for BatteryState {
    fn default() -> Self {
        Self::new(
            DEFAULT_LOW,
            DEFAULT_VERY_LOW,
            DEFAULT_CRITICAL,
            DEFAULT_REFRESH,
            VerbosityLevel::None,
        )
        .unwrap()
    }
}
