use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};
use chrono::{DateTime, Local};
use clap::Parser;
use sysinfo::System;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};

#[derive(Parser, Debug)]
#[command(name = "Rest Reminder")]
#[command(author = "Emil")]
#[command(version = "1.0")]
#[command(about = "Detects if you're working too long and reminds you to rest.", long_about = None)]
pub struct Args {
    #[arg(long, default_value = "D:\\", help = "Where to save the log file: c, d, desktop")]
    pub log_to: String,
}

#[derive(Debug, Clone)]
pub enum LogLocation {
    C,
    D,
}

impl LogLocation {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "c" => LogLocation::C,
            _ => LogLocation::D,
        }
    }
}

/// Non-stop working time
/// 
/// User can set their preferred non-stop working time manually.
const WORKING_TIME: u64 = 10;

/// Main function
pub fn run_rest_reminder(log_location: LogLocation) {
    let target_software = vec!["idea64.exe", "rustrover64.exe"];
    let mut sys = System::new_all();

    loop {
        sys.refresh_processes();
        let found = sys.processes()
            .values()
            .any(|process|
                target_software.iter()
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
                        target_software.iter()
                            .any(|software|
                                process.name().contains(software)));
                if !still_running {
                    println!("IDE closed, you stopped. Time being reset...");
                    break;
                }

                let elapsed = start_time.elapsed();
                // Set for 1h
                if elapsed.as_secs() >= WORKING_TIME {
                    println!("IDE still running, you need a break!");
                    pop_up();
                    log(start, Local::now(), log_location.clone());
                    break;
                }
            }
        } else {
            sleep(Duration::from_secs(10));
        }
    }
}

fn pop_up() {
    let title = widestring::U16CString::from_str(
        "REST REMINDEEEEEEEEEEEEEEEEER").unwrap();
    
    let message_string = format!("{WORKING_TIME} seconds NON-STOOOOOOOOOOOOP! YOU MUST BE TIRED! STAND UP AND TAKE A BREAK!!!!!!!");
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

fn log(start: DateTime<Local>, end: DateTime<Local>, log_location: LogLocation) {
    let duration = end - start;
    let log_line = format!(
        "[{} ~ {}] You worked for {:.2} minutes \n",
        start.format("%Y-%m-%d %H:%M:%S"),
        end.format("%Y-%m-%d %H:%M:%S"),
        duration.num_seconds() as f64 / 60.0
    );

    let path = get_log_path(log_location);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Cannot open log file!");

    file.write_all(log_line.as_bytes()).expect("Cannot write log file!");
}

fn get_log_path(location: LogLocation) -> PathBuf {
    match location {
        LogLocation::C => PathBuf::from("C:\\focus_log.txt"),
        LogLocation::D => PathBuf::from("D:\\focus_log.txt"),
    }
}