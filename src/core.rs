use clap::builder::ValueParser;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};
use chrono::{DateTime, Local, LocalResult, NaiveDate, NaiveDateTime, TimeZone};
use clap::{Parser, Subcommand};
use sysinfo::System;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};

#[derive(Parser, Debug)]
#[command(
    name = "Rest Reminder",
    author = "Emil Stampfly He",
    version = "1.0.0",
    about = "Detects if you're working too long and reminds you to rest.",
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    
    // Count precise working time
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
    Rest {
        #[arg(
            long,
            value_name = "PATH",
            default_value = r"D:\\focus_log.txt",
            value_parser = clap::value_parser!(PathBuf),
            help = "Where to save the log file",
        )]
        log_to: PathBuf,

        #[arg(
            long,
            value_name = "TIME",
            default_value_t = 3600,
            help = "How many seconds to work non stop before reminding",
        )]
        time: u64,

        #[arg(
            long,
            value_name = "APP",
            num_args = 1..,
            default_values = &["idea64.exe", "rustrover64.exe"],
            help = "What software(s) to detect",
        )]
        app: Vec<String>,
    },
}

fn parse_datetime_local(s: &str) -> Result<DateTime<Local>, String> {
    // To NaiveDateTime, no timezone
    let naive = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| format!("Failed to resolve: {}", e))?;

    // From NaiveDateTime to LocalDateTime
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

/// Main function
pub fn run_rest_reminder(mut log_location: PathBuf, time: u64, app: Vec<String>) {
    let mut sys = System::new_all();

    loop {
        sys.refresh_processes();
        let found = sys.processes()
            .values()
            .any(|process|
                app.iter()
                    .any(|software|
                        process.name().contains(software)));
        if found {
            let start = Local::now();
            println!("IDE detected, you are about to start working...");
            let start_time = Instant::now();

            loop {
                sleep(Duration::from_secs(10));
                sys.refresh_processes();
                let still_running = sys.processes()
                    .values()
                    .any(|process|
                        app.iter()
                            .any(|software|
                                process.name().contains(software)));
                if !still_running {
                    println!("IDE closed, you stopped. Time being reset...");
                    break;
                }

                let elapsed = start_time.elapsed();
                // Set for 1h
                if elapsed.as_secs() >= time {
                    println!("IDE still running, you need a break!");
                    pop_up(time);
                    log(start, Local::now(), &mut log_location);
                    break;
                }
            }
        } else {
            sleep(Duration::from_secs(10));
        }
    }
}

fn pop_up(time: u64) {
    let title = widestring::U16CString::from_str(
        "REST REMINDEEEEEEEEEEEEEEEEER").unwrap();

    let message_string = format!("{time} seconds NON-STOOOOOOOOOOOOP! YOU MUST BE TIRED! STAND UP AND TAKE A BREAK!!!!!!!");
    let message = widestring::U16CString::from_str(
        message_string.as_str())
        .unwrap();

    unsafe {
        MessageBoxW(
            HWND(0),
            PCWSTR(message.as_ptr()),
            PCWSTR(title.as_ptr()),
            MB_OK,
        );
    }
}

fn log(start: DateTime<Local>, end: DateTime<Local>, log_location: &mut PathBuf) {
    let mut path = log_location.to_path_buf();
    if path.is_dir() || log_location.to_string_lossy().ends_with(std::path::MAIN_SEPARATOR) {
        path.push("focus_log.txt");
    }

    let duration = end - start;
    let log_line = format!(
        "[{} ~ {}] You worked for {:.2} minutes \n",
        start.format("%Y-%m-%d %H:%M:%S"),
        end.format("%Y-%m-%d %H:%M:%S"),
        duration.num_seconds() as f64 / 60.0
    );


    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Cannot open log file!");

    file.write_all(log_line.as_bytes()).expect("Cannot write log file!");
}