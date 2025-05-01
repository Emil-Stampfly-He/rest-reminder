use clap::Parser;
use rest_reminder::cli::{Cli, Command};
use rest_reminder::core::run_rest_reminder;
use rest_reminder::plotter::plot;
use rest_reminder::statistics::{acc_work_time, acc_work_time_precise, single_day_work_time};

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Command::CountPrecise { log_location, start, end } => {
            let sec = acc_work_time_precise(log_location, start, end)
                .expect("Failed to calculate your work time.") as f64;
            println!("You worked {:?} seconds during this period of time", sec);
            println!("Or {:?} minutes", sec / 60.0);
            println!("Or {:?} hours", sec / 3600.0);
        }
        Command::Count { log_location, start_day, end_day } => {
            let sec = acc_work_time(log_location, start_day, end_day)
                .expect("Failed to calculate your work time.") as f64;
            println!("You worked {:?} seconds during these days", sec);
            println!("Or {:?} minutes", sec / 60.0);
            println!("Or {:?} hours", sec / 3600.0);
        }
        Command::CountSingleDay { log_location, day } => {
            let sec = single_day_work_time(log_location, day)
                .expect("Failed to calculate your work time.") as f64;
            println!("You worked {:?} seconds during this day", sec);
            println!("Or {:?} minutes", sec / 60.0);
            println!("Or {:?} hours", sec / 3600.0);
        }
        Command::Rest { log_to, time, app } => {
            run_rest_reminder(log_to, time, app);
        }
        Command::Plot {log_location, plot_location, start_day, end_day} => {
            plot(log_location, plot_location, start_day, end_day).
                expect("Failed to plot your working trend.");
        }
    }
}




