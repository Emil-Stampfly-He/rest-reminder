use clap::Parser;
use rest_reminder::{run_rest_reminder, Args};

fn main() {
    let args = Args::parse();
    
    run_rest_reminder(args.log_to, args.time);
}




