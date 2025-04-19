# Rest Reminder (Windows User Only Currently)

A small Rust-based Windows application that monitors whether IntelliJ IDEA or RustRover is running, and reminds you to take a break after one hour of continuous usage. It also logs your focused work sessions to a file.

## Features

- Monitors specific IDE processes (e.g., `idea64.exe`, `rustrover64.exe`)
- Tracks focused work time
- Displays a blocking system popup message after 1 hour of continuous work
- Logs start/end time and duration to a file
- Allows user to configure log file location via CLI options

## Working Sample

> ![Screenshot](Screenshot.png)

## How It Works

1. The program checks if IntelliJ IDEA or RustRover is currently running. You can add other processes, of course.
2. If either is detected, it starts tracking the time.
3. If 1 hour (if you want it to be shorter or longer, manually setting the time is possible) passes without closing the IDE, a system popup appears to remind you to take a break.
4. The session is logged to a file for record-keeping.

## Installation

There are two ways for you to install this application.

### 1. Download from Releases
If you do not wish to change anything, directly downloading from Releases is recommended. After downloading, run this command
in your terminal:
```aiignore
rest-reminder.exe --log-to d
```
`--log-to d` means the log file will be saved in your `D:\`. There's an alternate location you can choose:
```aiignore
rest-reminder.exe --log-to c
```
which will be saved in your `C:\`, should you run your terminal in administration.

### 2. Build from Source
If you do wish to DIY it for yourself, first, ensure you have Rust and Cargo installed:

```aiignore
git clone https://github.com/Emil-Stampfly-He/rest-reminder
```
Locations related code chunks:
```Rust
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

fn get_log_path(location: LogLocation) -> PathBuf {
    match location {
        LogLocation::C => PathBuf::from("C:\\focus_log.txt"),
        LogLocation::D => PathBuf::from("D:\\focus_log.txt"),
    }
}
```
Working time related code line:
```Rust
const WORKING_TIME: u64 = 10;
```
Listened processes related code lines:
```Rust
pub fn run_rest_reminder(log_location: LogLocation) {
    let target_software = vec!["idea64.exe", "rustrover64.exe"];
    // Omitted
}
```
After you change everything, make sure it works, then:
```aiignore
cargo build --release
```
The final .exe file should appear in your `\target\release`directory.

