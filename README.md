# Rest Reminder (Windows User Only Currently)
简体中文：[README_zh.md](./README-zh_CN.md).

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
rest-reminder.exe --log-to <file_location> --time <time>
```
If you did not indicate your focus_log.txt file location, it will be set to be `D:\` in default. The default work time is 3600 seconds (1 hour).
For blank `<time>`, you need to indicate in second, not minute or hour.

**Reminder: DO NOT add `focus_log.txt` after your file location!** For example:
* "D:\\": allowed
* "D:\\focus_log.txt": **NOT** allowed
* "D:\\name": **NOT** allowed, since you missed a `\` suffix

### 2. Build from Source
If you do wish to DIY it for yourself, first, ensure you have Rust and Cargo installed:

```aiignore
git clone https://github.com/Emil-Stampfly-He/rest-reminder
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

