mod battery_state;

// use battery_state::*;
use battery_state::{BatteryState, VerbosityLevel};
use clap::Clap;

#[derive(Clap)]
#[clap(
    version = "0.1.0",
    author = "Lucien Cartier-Tilet <lucien@phundrak.com>"
)]
struct Opts {
    #[clap(short, long, default_value = "25")]
    low: f32,

    #[clap(short = "L", long, default_value = "15")]
    very_low: f32,

    #[clap(short, long, default_value = "10")]
    critical: f32,

    #[clap(short, long = "refresh-rate", default_value = "5")]
    refresh_rate: u64,

    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("Low battery: {}%", opts.low);
    println!("Very low battery: {}%", opts.very_low);
    println!("Critical battery: {}%", opts.critical);
    println!("Refresh rate: {}s", opts.refresh_rate);
    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        _ => println!("Lots of verbose info"),
    }

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
