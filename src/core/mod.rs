use colored::Colorize;
use crate::cli::Command;
use crate::core::core::run_rest_reminder;
use crate::plugin::template::generate_plugin_template;
use crate::statistic::plotter::plot;
use crate::statistic::statistics::{acc_work_time, acc_work_time_precise, single_day_work_time};
use crate::web::web::spawn_web_server;

pub mod core;
pub mod interactive;

pub async fn execute_command(cmd: Command) {
    match cmd {
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
            println!("{}", "Starting Rest Reminder...".bright_yellow().bold());
            run_rest_reminder(log_to, time, app).await;
        }
        Command::Plot {log_location, plot_location, start_day, end_day} => {
            println!("{}", "Generating plot...".bright_yellow().bold());
            plot(log_location, plot_location, start_day, end_day)
                .expect("Failed to plot your working trend.");
            println!("{}", "Plot generated successfully!".bright_green().bold());
        }
        Command::Gen { name } => {
            println!("{}", "Generating plugin template...".bright_yellow().bold());
            generate_plugin_template(name.as_str()).await;
        }
        Command::Web {} => {
            println!("{}", "Starting web server...".bright_yellow().bold());
            spawn_web_server().await;
            println!("{} {}", "Web server started:".bright_green().bold(), "http://localhost:60606".white().bold());
        }
    }
}