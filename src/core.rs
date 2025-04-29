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
#[command(
    name = "Rest Reminder",
    author = "Emil Stampfly He",
    version = "0.3.0",
    about = "Detects if you're working too long and reminds you to rest.",
    long_about = None,
)]
pub struct Args {
    #[arg(
        long,                                     // generate --log-to
        value_name = "PATH",                      // show <PATH> in help
        default_value = r"D:\\focus_log.txt",     // default value
        value_parser = clap::value_parser!(PathBuf),
        help = "Where to save the log file"
    )]
    pub log_to: PathBuf,

    #[arg(
        long,                                     // generate --time
        value_name = "TIME",                      // show <PATH> in help
        default_value_t = 3600,                   // default value
        help = "How many seconds to work non stop before reminding"
    )]
    pub time: u64,

    #[arg(
        long,
        value_name = "APP",
        num_args = 1..,                                         // at least 1, no limit
        default_values = &["idea64.exe", "rustrover64.exe"],    // default value
        help = "What software(s) to detect"
    )]
    pub app: Vec<String>,
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