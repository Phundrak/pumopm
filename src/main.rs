mod battery_state;

// use battery_state::*;
use battery_state::{BatteryState, VerbosityLevel};
use clap::Clap;

#[derive(Clap)]
#[clap(
    version = "0.1.1",
    author = "Lucien Cartier-Tilet <lucien@phundrak.com>"
)]
struct Opts {
    #[clap(short, long, default_value = "25")]
    low: u8,

    #[clap(short = "L", long, default_value = "15")]
    very_low: u8,

    #[clap(short, long, default_value = "10")]
    critical: u8,

    #[clap(short, long = "refresh-rate", default_value = "5")]
    refresh_rate: u64,

    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut battery = BatteryState::new(
        opts.low,
        opts.very_low,
        opts.critical,
        opts.refresh_rate,
        match opts.verbose {
            0 => VerbosityLevel::None,
            1 => VerbosityLevel::Some,
            _ => VerbosityLevel::Lots,
        },
    )
    .unwrap();
    loop {
        battery.update();
    }
}
