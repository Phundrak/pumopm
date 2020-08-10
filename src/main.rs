mod battery_state;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");

use clap::{App, Arg};
use std::{thread, time::Duration};

macro_rules! get_arg_value {
    ($args:ident, $arg:literal, $type:ty, $default:ident) => {
        match $args.value_of($arg) {
            Some(value) => match value.parse::<$type>() {
                Ok(ret) => ret,
                Err(e) => {
                    eprintln!(
                        "Error parsing {}, using default. Error: {}",
                        value, e
                    );
                    $default
                }
            },
            None => $default,
        }
    };
}

fn main() {
    let arguments = App::new("PumoPM")
        .version(VERSION.unwrap_or("unknown"))
        .author(AUTHORS.unwrap_or("Lucien Cartier-Tilet <lucien@phundrak.com>"))
        .about("Tiny custom power manager")
        .arg(Arg::with_name("low-battery")
             .short("l")
             .long("low")
             .value_name("LOW")
             .help("Level at which the battery’s level is considered low")
             .takes_value(true))
        .arg(Arg::with_name("very-low-battery")
             .short("L")
             .long("very-low")
             .value_name("VERY LOW")
             .help("Level at which the battery’s level is considered very low")
             .takes_value(true))
        .arg(Arg::with_name("critical-battery")
             .short("c")
             .long("critical")
             .value_name("CRITICAL")
             .help("Level at which the battery’s level is considered critical")
             .takes_value(true))
        .arg(Arg::with_name("refresh-rate")
             .short("r")
             .long("refresh-rate")
             .value_name("REFRESH RATE")
             .help("How often should the battery’s levels be read (in seconds)")
             .takes_value(true))
        .get_matches();

    use battery_state::{
        DEFAULT_CRITICAL, DEFAULT_LOW, DEFAULT_REFRESH, DEFAULT_VERY_LOW,
    };
    let low_battery =
        get_arg_value!(arguments, "low-battery", f32, DEFAULT_LOW);
    let very_low_battery =
        get_arg_value!(arguments, "very-low-battery", f32, DEFAULT_VERY_LOW);
    let critical_battery =
        get_arg_value!(arguments, "critical-battery", f32, DEFAULT_CRITICAL);
    let refresh_rate =
        get_arg_value!(arguments, "refresh-rate", u8, DEFAULT_REFRESH);

    // let mut battery = battery_state::BatteryState::new(low_battery);
    let mut battery = battery_state::BatteryState::new(
        low_battery,
        very_low_battery,
        critical_battery,
        refresh_rate,
    )
    .unwrap();
    loop {
        thread::sleep(Duration::from_secs(5));
        battery.update();
    }
}
