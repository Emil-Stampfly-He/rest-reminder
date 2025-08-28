use std::collections::{HashMap};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;
use chrono::{DateTime, Local};
use rand::Rng;
use sysinfo::{ProcessesToUpdate, System};
use tokio::time::{Duration, interval};
use colored::*;
use crate::plugin::plugins::{PluginContext, PluginManager};

// Windows specific imports
#[cfg(windows)]
use {
    windows::core::PCWSTR,
    windows::Win32::Foundation::HWND,
    windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK},
    widestring::U16CString,
};

#[derive(Debug)]
enum WorkSessionResult {
    CtrlCPressed,
    TimeReached,
    ProcessEnded,
}

pub async fn run_rest_reminder(mut log_location: PathBuf, time: u64, app: Vec<String>) {
    let mut sys = System::new_all();
    let mut process_check_interval = interval(Duration::from_secs(1));
    let mut msg_undiscovered_display_interval = interval(Duration::from_secs(2));

    let mut plugin_manager = PluginManager::new().unwrap_or_else(|e| {
        println!("{} {}", "Failed to initialize plugin manager:".bright_red().bold(), e.to_string().red());
        PluginManager::new().unwrap()
    });

    if let Err(e) = plugin_manager.load_plugins("plugins") {
        println!("{} {}", "Failed to load plugins:".bright_red().bold(), e.to_string().red());
    }
    
    // Trigger init hook
    let init_context = PluginContext::new("Rest Reminder initialized", 0);
    if let Err(e) = plugin_manager.trigger_hook("on_init", &init_context) {
        println!("{} {}", "Plugin hook error:".bright_red(), e.to_string().red());
    }
    
    // Trace current state
    let mut last_found_state = false;

    loop {
        tokio::select! {
            // Handle Ctrl+C gracefully
            press_ctrl_c_result = tokio::signal::ctrl_c() => {
                match press_ctrl_c_result {
                    Ok(_) => println!("{}", "Stopped monitoring".bright_yellow().bold()),
                    Err(_) => panic!("Failed to start work session")
                }
                return;
            }
            
            // Regular process checking
            _ = process_check_interval.tick() => {
                sys.refresh_processes(ProcessesToUpdate::All, true);
                let found = sys.processes()
                    .values()
                    .any(|process|
                        app.iter()
                            .any(|software|
                                process.name().to_str().unwrap().contains(software)));
                
                // Reset intervals when state changes
                if found != last_found_state {
                    if !found { msg_undiscovered_display_interval = interval(Duration::from_secs(2)); }
                    last_found_state = found;
                }
                
                if found {
                    let start = Local::now();
                    println!("{}", "Process(es) detected, you are about to start working...".bright_green().bold());
                    
                    // Trigger work start hook
                    let work_start_context = PluginContext::new("Work session started", 0);
                    if let Err(e) = plugin_manager.trigger_hook("on_work_start", &work_start_context) {
                        println!("{} {}", "Plugin hook error:".bright_red(), e.to_string().red());
                    }
                    
                    let start_time = Instant::now();
                    
                    // Monitor working session
                    let work_session_result = monitor_work_session(&mut sys, &app, time, start_time).await;
                    match work_session_result {
                        WorkSessionResult::CtrlCPressed => {
                            log(start, Local::now(), &mut log_location);
                            println!("{}", "Stopped monitoring".bright_yellow().bold());
                            return;
                        }
                        WorkSessionResult::TimeReached => {
                            println!("{}", "Process(es) still running, you need a break!".bright_red().bold());
                            
                            // Trigger break hook
                            let break_context = PluginContext::new("Time to take a break!", time);
                            if let Err(e) = plugin_manager.trigger_hook("on_break_reminder", &break_context) {
                                println!("{} {}", "Plugin hook error:".bright_red(), e.to_string().red());
                            }
                            
                            pop_up(time).await;
                            log(start, Local::now(), &mut log_location);
                        }
                        WorkSessionResult::ProcessEnded => {
                            log(start, Local::now(), &mut log_location);
                            println!("{}", "Process(es) ended, you finally decide to rest...".bright_blue().bold());
                            last_found_state = false;
                        }
                    }
                }
            }

            _ = msg_undiscovered_display_interval.tick(), if !last_found_state => {
                println!("{}", "No processes detected, you are resting...".cyan());
            }
        }
    }
}

async fn monitor_work_session(
    sys: &mut System, 
    app: &[String], 
    time: u64, 
    start_time: Instant
) -> WorkSessionResult {
    let mut heartbeat = interval(Duration::from_secs(2));
    
    loop {
        tokio::select! {
            // Handle Ctrl+C during work session
            _ = tokio::signal::ctrl_c() => {
                return WorkSessionResult::CtrlCPressed;
            }
            
            // Regular heartbeat check
            _ = heartbeat.tick() => {
                sys.refresh_processes(ProcessesToUpdate::All, true);
                let still_running = sys.processes()
                    .values()
                    .any(|process|
                        app.iter()
                            .any(|software|
                                process.name().to_str().unwrap().contains(software)));
                
                if !still_running {
                    return WorkSessionResult::ProcessEnded;
                }

                let elapsed = start_time.elapsed();
                if elapsed.as_secs() >= time {
                    return WorkSessionResult::TimeReached;
                }
            }
        }
    }
}

async fn pop_up(time: u64) {
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
        None => panic!("Index out of bounds!")
    };

    #[cfg(windows)]
    show_popup_windows(&message_string).await;

    #[cfg(target_os = "macos")]
    show_popup_macos(&message_string).await;

    #[cfg(not(any(windows, target_os = "macos")))]
    {
        println!("{}", format!("ALERT: {}", message_string).bright_red().bold().on_yellow());
        println!("{}", format!("Rest reminder: You have been working continuously for {} seconds, it's time to take a break!", time).bright_magenta().bold());
    }
}

#[cfg(windows)]
async fn show_popup_windows(message: &str) {
    let title = U16CString::from_str("REST REMINDEEEEEEEEEEEEEEEEER").unwrap();
    let message = U16CString::from_str(message).unwrap();
    
    // Windows API is synchronous, so run it in a background thread
    tokio::task::spawn_blocking(move || {
        unsafe {
            MessageBoxW(
                HWND(0),
                PCWSTR(message.as_ptr()),
                PCWSTR(title.as_ptr()),
                MB_OK,
            );
        }
    }).await.expect("Failed to show popup");
}

#[cfg(target_os = "macos")]
async fn show_popup_macos(message: &str) {
    let script = format!(
        r#"display dialog "{}" with title "REST REMINDEEEEEEEEEEEEEEEEER" buttons {{"OK"}} default button "OK""#,
        message.replace("\"", "\\\"")
    );
    
    let _ = tokio::process::Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .status()
        .await;
}

fn log(start: DateTime<Local>, end: DateTime<Local>, log_location: &mut PathBuf) {
    let mut path = log_location.to_path_buf();
    if path.is_dir() || log_location.to_string_lossy().ends_with(std::path::MAIN_SEPARATOR) {
        path.push("../../focus_log.txt");
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
    println!("{} {}", "Logging to".bright_green().bold(), log_location.to_string_lossy());
}