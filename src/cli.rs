use clap::builder::ValueParser;
use std::path::PathBuf;
use chrono::{DateTime, Local, LocalResult, NaiveDate, NaiveDateTime, TimeZone};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "Rest Reminder",
    author = "Emil Stampfly He",
    version = "1.1.0",
    about = "Detects if you're working too long and reminds you to rest.",
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    
    // Statistics
    #[command(name = "count-precise")]
    CountPrecise {
        #[arg(
            value_name = "PATH", 
            default_value = r"D:\\focus_log.txt",
            value_parser = ValueParser::path_buf()
        )]
        log_location: PathBuf,

        #[arg(
            value_name = "START",
            help = "Format: YYYY-MM-DD HH:MM:SS",
            value_parser = parse_datetime_local,
        )]
        start: DateTime<Local>,

        #[arg(
            value_name = "END",
            help = "Format: YYYY-MM-DD HH:MM:SS",
            value_parser = parse_datetime_local,
        )]
        end: DateTime<Local>,
    },

    #[command(name = "count")]
    Count {
        #[arg(
            value_name = "PATH", 
            default_value = r"D:\\focus_log.txt",
            value_parser = ValueParser::path_buf()
        )]
        log_location: PathBuf,

        #[arg(
            value_name = "START",
            help = "Format: YYYY-MM-DD HH:MM:SS",
            value_parser = parse_datetime_local_day,
        )]
        start_day: DateTime<Local>,

        #[arg(
            value_name = "END",
            help = "Format: YYYY-MM-DD HH:MM:SS",
            value_parser = parse_datetime_local_day,
        )]
        end_day: DateTime<Local>,
    },

    #[command(name = "count-single-day")]
    CountSingleDay {
        #[arg(
            value_name = "PATH", 
            default_value = r"D:\\focus_log.txt",
            value_parser = ValueParser::path_buf()
        )]
        log_location: PathBuf,

        #[arg(
            value_name = "START",
            help = "Format: YYYY-MM-DD HH:MM:SS",
            value_parser = parse_datetime_local_day,
        )]
        day: DateTime<Local>,
    },

    // Rest reminder
    #[command(name = "rest")]
    Rest {
        #[arg(
            value_name = "PATH",
            default_value = r"D:\\focus_log.txt",
            value_parser = clap::value_parser!(PathBuf),
            help = "Where to save the log file",
        )]
        log_to: PathBuf,

        #[arg(
            value_name = "TIME",
            default_value_t = 3600,
            help = "How many seconds to work non stop before reminding",
        )]
        time: u64,

        #[arg(
            value_name = "APP",
            num_args = 1..,
            default_values = &["idea64.exe", "rustrover64.exe"],
            help = "What software(s) to detect",
        )]
        app: Vec<String>,
    },
    
    // Plotting
    #[command(name = "plot")]
    Plot {
        #[arg(
            value_name = "PATH",
            default_value = r"D:\\focus_log.txt",
            value_parser = clap::value_parser!(PathBuf),
            help = "Where to save the log file",
        )]
        log_location: PathBuf,

        #[arg(
            value_name = "PATH",
            default_value = r"D:\\plot.png",
            value_parser = clap::value_parser!(PathBuf),
            help = "Where to save the log file",
        )]
        plot_location: PathBuf,
        
        #[arg(
            value_name = "START",
            help = "Format: YYYY-MM-DD HH:MM:SS",
            value_parser = parse_datetime_local_day,
        )]
        start_day: DateTime<Local>,

        #[arg(
            value_name = "END",
            help = "Format: YYYY-MM-DD HH:MM:SS",
            value_parser = parse_datetime_local_day,
        )]
        end_day: DateTime<Local>,
    },
}

fn parse_datetime_local(s: &str) -> Result<DateTime<Local>, String> {
    // To NaiveDateTime
    let naive = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| format!("Failed to resolve: {}", e))?;

    // To LocalDateTime
    match Local.from_local_datetime(&naive) {
        LocalResult::Single(dt) => Ok(dt),
        LocalResult::Ambiguous(dt1, _dt2) => Ok(dt1),
        LocalResult::None => Err(format!("Time '{}' is invalid in this timezone", s)),
    }
}

fn parse_datetime_local_day(s: &str) -> Result<DateTime<Local>, String> {
    // To NaiveDate
    let naive_date = NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| format!("Cannot resolve '{}': {}", s, e))?;

    let naive_dt = naive_date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| format!("Cannot generate timestamp for '{}'", s))?;

    // Map to local timezone
    match Local.from_local_datetime(&naive_dt) {
        LocalResult::Single(dt)    => Ok(dt),
        LocalResult::Ambiguous(dt, _) => Ok(dt),
        LocalResult::None => Err(format!("Date is invalid in local timezone '{}'", s)),
    }
}