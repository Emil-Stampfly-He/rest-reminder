use clap::Parser;
use rest_reminder::{run_rest_reminder, Args, LogLocation};

fn main() {
    let args = Args::parse();
    let log_location = LogLocation::from_str(&args.log_to);
    
    run_rest_reminder(log_location);
}




