use std::collections::{HashMap};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};
use chrono::{DateTime, Local};
use rand::Rng;
use sysinfo::System;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};

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
            println!("Process(es) detected, you are about to start working...");
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
                    println!("Process(es) ended, you finally decide to rest...");
                    break;
                }

                let elapsed = start_time.elapsed();
                // Set for 1h
                if elapsed.as_secs() >= time {
                    println!("Process(es) still running, you need a break!");
                    pop_up(time);
                    log(start, Local::now(), &mut log_location);
                    break;
                }
            }
        } else {
            println!("No process(es) detected, you are resting...");
            sleep(Duration::from_secs(10));
        }
    }
}

fn pop_up(time: u64) {
    let title = widestring::U16CString::from_str(
        "REST REMINDEEEEEEEEEEEEEEEEER").unwrap();
    
    let slogans = HashMap::from([
        (0, format!("{time} seconds NON-STOOOOOOOOOOOOP! YOU MUST BE TIRED! STAND UP AND TAKE A BREAK!!!!!!!")),
        (1, format!("{time} seconds OF UNSTOPPABLE GRIND! YOUR LEGS ARE CRYING FOR A BREAK! STAND UP AND SHAKE IT OFF!!!")),
        (2, format!("{time} seconds STRAIGHT LIKE A NINJA! YOUR BACK IS REBELLING! POWER UP WITH A QUICK STAND-UP BREAK!!!")),
        (3, format!("{time} seconds WITHOUT PAUSE! ALERT: MUSCLES ON STRIKE! RISE AND RELEASE WITH A STRETCH!!!")),
        (4, format!("{time} seconds NONSTOP MODE ENGAGED! WARNING: BRAIN FOG IMMINENT! HIT THE PAUSE AND STAND TALL!!!")),
        (5, format!("{time} seconds AND COUNTING! MISSION: TAKE A BREAK! DEPLOY YOUR LEGS FOR A STAND-UP MISSION!!!")),
    ]);

    let rng = rand::rng().random_range(0..slogans.len());
    let message_string = match slogans.get(&rng) {
        Some(slogan) => slogan.to_string(),
        None => panic!("No slogan found!")
    };
    
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