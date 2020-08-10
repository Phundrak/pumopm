mod battery_state;

use std::{io, thread};
use std::{process::Command, time::Duration};

fn main() {
    let mut battery = battery_state::BatteryState::default();

    loop {
        thread::sleep(Duration::from_secs(5));
        battery.update();
    }

    // loop {
    //     thread::sleep(Duration::from_secs(5));
    //     manager.refresh(&mut battery)?;
    //     let charge = battery.state_of_charge().value;

    //     // let charge = charge.value as f32 * 100.0;

    //     // Notification::new()
    //     //     .summary("Battery charge")
    //     //     .body(format!("Current battery level is {}", charge).as_str())
    //     //     .hint(Hint::Category("battery".to_owned()))
    //     //     .urgency(Urgency::Low)
    //     //     .show()
    //     //     .unwrap();

    //     // let result = match charge.value as f32 * 100.0 {
    //     //     x if x < 5.0 => Command::new("sh")
    //     //         .arg("systemctl")
    //     //         .arg("hibernate")
    //     //         .output(),
    //     //     x if x < 10.0 => {
    //     //         Notification::new()
    //     //             .summary("Battery very low")
    //     //             .body(format!("Current battery level is {}", x).as_str())
    //     //             .hint(Hint::Category("battery".to_owned()))
    //     //             .urgency(Urgency::Critical)
    //     //             .show().unwrap();
    //     //     },
    //     //     _ => Ok(()),
    //     // };

    // }
}
