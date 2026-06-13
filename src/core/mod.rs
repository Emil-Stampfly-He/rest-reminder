use crate::cli::Command;
use crate::core::core::run_rest_reminder;
use crate::plugin::template::generate_plugin_template;
use crate::statistic::plotter::plot;
use crate::statistic::statistics::{
    acc_work_time_for_task, acc_work_time_precise_for_task, single_day_work_time_for_task,
    task_work_time_summary,
};
use crate::web::web::spawn_web_server;
use colored::Colorize;

pub mod core;
pub mod interactive;

pub async fn execute_command(cmd: Command) {
    match cmd {
        Command::CountPrecise {
            log_location,
            start,
            end,
            task,
        } => {
            let sec = acc_work_time_precise_for_task(log_location, start, end, task.as_deref())
                .expect("Failed to calculate your work time.") as f64;
            println!("You worked {:?} seconds during this period of time", sec);
            println!("Or {:?} minutes", sec / 60.0);
            println!("Or {:?} hours", sec / 3600.0);
        }
        Command::Count {
            log_location,
            start_day,
            end_day,
            task,
        } => {
            let sec = acc_work_time_for_task(log_location, start_day, end_day, task.as_deref())
                .expect("Failed to calculate your work time.") as f64;
            println!("You worked {:?} seconds during these days", sec);
            println!("Or {:?} minutes", sec / 60.0);
            println!("Or {:?} hours", sec / 3600.0);
        }
        Command::CountSingleDay {
            log_location,
            day,
            task,
        } => {
            let sec = single_day_work_time_for_task(log_location, day, task.as_deref())
                .expect("Failed to calculate your work time.") as f64;
            println!("You worked {:?} seconds during this day", sec);
            println!("Or {:?} minutes", sec / 60.0);
            println!("Or {:?} hours", sec / 3600.0);
        }
        Command::CountByTask {
            log_location,
            start_day,
            end_day,
        } => {
            let summaries = task_work_time_summary(log_location, start_day, end_day)
                .expect("Failed to calculate your task summary.");
            if summaries.is_empty() {
                println!("No work sessions found during these days");
            } else {
                for summary in summaries {
                    println!(
                        "{}: {} seconds ({:.2} minutes, {:.2} hours)",
                        summary.task,
                        summary.seconds,
                        summary.seconds as f64 / 60.0,
                        summary.seconds as f64 / 3600.0
                    );
                }
            }
        }
        Command::Rest {
            log_to,
            time,
            app,
            task,
        } => {
            println!("{}", "Starting Rest Reminder...".bright_yellow().bold());
            run_rest_reminder(log_to, time, app, task, None).await;
        }
        Command::Plot {
            log_location,
            plot_location,
            start_day,
            end_day,
        } => {
            println!("{}", "Generating plot...".bright_yellow().bold());
            plot(log_location, plot_location, start_day, end_day)
                .expect("Failed to plot your working trend.");
            println!("{}", "Plot generated successfully!".bright_green().bold());
        }
        Command::Gen { name } => {
            println!("{}", "Generating plugin template...".bright_yellow().bold());
            generate_plugin_template(name.as_str())
                .await
                .expect("Failed to generate plugin template.");
        }
        Command::Web {} => {
            println!("{}", "Starting web server...".bright_yellow().bold());
            let handle = spawn_web_server().await;
            println!(
                "{} {}",
                "Web server started:".bright_green().bold(),
                "http://localhost:60606".white().bold()
            );
            handle
                .join()
                .expect("Failed to join web server thread")
                .expect("Web server failed");
        }
    }
}
