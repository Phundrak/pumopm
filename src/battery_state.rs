use std::cmp::PartialOrd;

use notify_rust::{Hint, Notification, Urgency};

#[derive(PartialEq, PartialOrd, Eq, Debug)]
pub enum VerbosityLevel {
    None = 0,
    Some = 1,
    Lots = 2,
}

#[derive(Debug)]
pub struct BatteryState {
    manager: battery::Manager,
    battery: battery::Battery,
    refresh_rate: u64,

    low_level: u8,
    very_low_level: u8,
    critical_level: u8,

    is_triggered_low: bool,
    is_triggered_very_low: bool,

    verbosity: VerbosityLevel,
}

pub const DEFAULT_LOW: u8 = 25_u8;
pub const DEFAULT_VERY_LOW: u8 = 15_u8;
pub const DEFAULT_CRITICAL: u8 = 10_u8;
pub const DEFAULT_REFRESH: u64 = 5;

macro_rules! trigger_warning {
    ($message:literal, $urgency:ident, $trigger:expr, $battery:expr, $verbosity:expr, $self:expr) => {
        if $trigger {
            return;
        }
        $trigger = true;
        let level = ($battery.state_of_charge().value * 100_f32) as u8;
        let message = format!($message, $self.remaining_time($self.battery.time_to_full()), level);
        match $verbosity {
            VerbosityLevel::None => {}
            _ => println!("{}", message),
        }
        Notification::new()
            .summary("Low battery")
            .body(message.as_str())
            .hint(Hint::Category("battery".to_owned()))
            .urgency($urgency)
            .show()
            .unwrap();
    };
}

impl BatteryState {
    /// Create and initialize new `BatteryState` object
    pub fn new(
        low_level: u8,
        mut very_low_level: u8,
        mut critical_level: u8,
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

        // Keep values safe
        let low_level = low_level.min(95_u8).max(5_u8);
        if very_low_level > low_level {
            very_low_level = u8::max(low_level - 1_u8, 5_u8)
        };
        if critical_level > very_low_level {
            critical_level = u8::max(very_low_level - 1_u8, 5_u8)
        };

        match verbosity {
            VerbosityLevel::None => {}
            _ => {
                println!("Low battery: {}%", low_level);
                println!("Very low battery: {}%", very_low_level);
                println!("Critical battery: {}%", critical_level);
                println!("Refresh rate: {}s", refresh_rate);
                match verbosity {
                    VerbosityLevel::Some => println!("Some verbose info"),
                    _ => println!("Lots of verbose info"),
                }
            }
        }

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

    /// Reset current progress toward an empty battery
    fn reset_levels(&mut self) {
        self.is_triggered_low = false;
        self.is_triggered_very_low = false;
    }

    /// Get level of charge of the battery
    fn get_charge(&self) -> u8 {
        (self.battery.state_of_charge().value * 100_f32) as u8
    }

    pub fn remaining_time(&self, time: Option<battery::units::Time>) -> String {
        match time {
            Some(e) => {
                let time = e.value as u64;
                let hours = time / 3600;
                let minutes = (time % 3600) / 60;
                let seconds = time % 60;
                format!("{:01}:{:02}:{:02}", hours, minutes, seconds)
            }
            None => {
                eprintln!("Couldn’t read remaining time");
                String::from("unknown remaining time")
            }
        }
    }

    /// Warn the user once about low battery
    fn trigger_low(&mut self) {
        use Urgency::Normal;
        trigger_warning!(
            "Battery level is low! {} left ({}%)",
            Normal,
            self.is_triggered_low,
            self.battery,
            self.verbosity,
            &self
        );
    }

    /// Warn the user once about very low battery
    fn trigger_very_low(&mut self) {
        use Urgency::Critical;
        trigger_warning!(
            "Battery level is low! {} left ({}%)",
            Critical,
            self.is_triggered_very_low,
            self.battery,
            self.verbosity,
            &self
        );
    }

    fn trigger_critical(&mut self) {
        use std::process::Command;
        let out = Command::new("systemctl")
            .arg("suspend")
            .output()
            .expect("process failed to execute");
        eprintln!("{}", out.status);
        eprintln!(
            "{}",
            String::from_utf8(out.stderr)
                .unwrap_or(String::from("Could not read stderr"))
        );
        eprintln!(
            "{:?}",
            String::from_utf8(out.stdout)
                .unwrap_or(String::from("Could not read stdin"))
        );
        loop {
            self.manager.refresh(&mut self.battery).unwrap();
            use std::{thread, time::Duration};
            thread::sleep(Duration::from_secs(self.refresh_rate));
            if self.battery.state() == battery::State::Charging {
                break;
            }
        }
    }

    pub fn update(&mut self) {
        self.manager.refresh(&mut self.battery).unwrap();

        let charge = self.get_charge();

        if self.verbosity >= VerbosityLevel::Some {
            match self.battery.state() {
                battery::State::Charging => println!(
                    "Charging: {}%, time left: {}",
                    charge,
                    self.remaining_time(self.battery.time_to_full())
                ),
                battery::State::Discharging => println!(
                    "Discharging: {}%, time left: {}",
                    charge,
                    self.remaining_time(self.battery.time_to_empty())
                ),
                battery::State::Full => println!("Full"),
                battery::State::Empty => println!("Empty"),
                _ => eprintln!("Error: unknown battery state"),
            }
        }
        if self.verbosity == VerbosityLevel::Lots {
            eprintln!("====\nDebug self:\n{:?}\n====", self);
        }

        match self.battery.state() {
            battery::State::Discharging | battery::State::Empty => {
                if !self.is_triggered_low && charge <= self.low_level {
                    self.trigger_low();
                } else if !self.is_triggered_very_low
                    && charge <= self.very_low_level
                {
                    self.trigger_very_low();
                } else if charge <= self.critical_level {
                    self.trigger_critical();
                }
            }
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
