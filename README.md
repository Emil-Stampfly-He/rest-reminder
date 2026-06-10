# Rest Reminder

![Lines of Code](https://img.shields.io/endpoint?url=https://Emil-Stampfly-He.github.io/rest-reminder/badge.json)

Simplified Chinese: [README-zh_CN.md](./README-zh_CN.md)

Rest Reminder is a Rust desktop utility for monitoring focused work sessions. It watches selected applications, reminds you to take a break after a configurable continuous work interval, records sessions to `focus_log.txt`, calculates accumulated work time, and generates work trend charts.

The project now includes both a command-line workflow and a local web UI.

## Features

- Cross-platform support for Windows, macOS, and Linux-like fallback environments.
- Process monitoring for selected applications.
- Native break reminders:
  - Windows: MessageBox
  - macOS: AppleScript dialog
  - Other platforms: terminal alert output
- Work session logging to `focus_log.txt`.
- Work time statistics by date range, single day, or precise timestamp range.
- Work trend chart generation as a PNG image.
- Local web UI at `http://localhost:60606`.
- Web UI supports English, Simplified Chinese, Traditional Chinese, Japanese, and French.
- Native file/folder pickers in the web UI for log directory, log file, and chart output path.
- Running-process selector in the web UI, so users can search and pick process names instead of looking them up manually.
- Monitor status panel with elapsed time, pause/resume controls, and a stop button.
- Saved web UI preferences for log paths, reminder interval, and monitored apps.
- Recent log preview and generated chart preview in the browser.
- Optional task labels for new work sessions, with task-filtered statistics in the CLI and Web UI.
- Python plugin hooks for custom automation on app initialization, work start, and break reminder.

## Screenshots

### Windows

> ![Screenshot](pics/Screenshot.png)
> ![Working Trend](pics/example.png)

### macOS

> ![Screenshot](pics/example_macOS.png)
> ![Working Trend](pics/example.png)

Sample log file: [`focus_log.txt`](focus_log.txt)

## How It Works

1. Rest Reminder checks whether any selected process is running.
2. When a selected process is detected, it starts a work session timer.
3. If the process keeps running past the configured interval, Rest Reminder shows a break reminder.
4. The work session is appended to `focus_log.txt`.
5. Statistics commands and the web UI read that log file to calculate work time.
6. The plot command and web UI can generate a work trend chart from the log file.

## Recommended: Web UI

Start the local web server:

```bash
cargo run -- web
```

Then open:

```text
http://localhost:60606
```

The web UI includes three panels:

- **Start monitoring**: choose a log directory, work interval, and applications to monitor.
- **Work statistics**: calculate total work time by date range, single day, or precise time range, optionally filtered by task label.
- **Generate chart**: choose a log file and save location for a work trend PNG.

### Web UI conveniences

- Use the language selector in the header to switch between English, Simplified Chinese, Traditional Chinese, Japanese, and French.
- Use **Browse** buttons to open native folder/file selection dialogs.
- Use the monitored-app search box to search currently running processes.
- Click a process from the dropdown to add it.
- If an app is not currently running, type its process name and press Enter to add it manually.
- Add a task label such as `coding`, `reading`, or `meeting` to new work sessions.
- See whether monitoring is currently running, how long it has been running, and which apps are being watched.
- Pause, resume, or stop the current monitor from the web UI. Paused time is not counted as work.
- Reopen the page with your previous log paths, interval, and monitored apps restored automatically.
- Preview recent log entries before running statistics.
- Preview the generated PNG chart directly in the browser after plotting.

## Interactive Mode

Running the binary without arguments opens interactive mode:

```bash
cargo run
```

Available commands include:

```text
rest
count
count-single-day
count-precise
count-by-task
plot
web
help
exit
```

## CLI Usage

You can also run commands directly.

### Start Monitoring

```bash
cargo run -- rest -l <LOG_DIRECTORY> -t <SECONDS> -a <APP_1> <APP_2> ... --task <TASK>
```

Examples:

```bash
cargo run -- rest -l ~/Desktop -t 3600 -a "Cursor" "Xcode"
cargo run -- rest -l D:\ -t 3600 -a Code.exe Notion.exe --task coding
```

Notes:

- `-l` for `rest` expects a directory. Rest Reminder writes `focus_log.txt` there.
- `-t` is in seconds. The default is `3600`.
- `-a` accepts one or more process names.
- `--task` is optional. When provided, new sessions are stored with that task label.
- Defaults vary by platform:
  - Windows: `idea64.exe`, `rustrover64.exe`, `Code.exe`
  - macOS: `IntelliJ IDEA`, `RustRover`, `Cursor`, `Xcode`
  - Other platforms: `idea`, `rustrover`, `code`

### Count Date Range

```bash
cargo run -- count -l <LOG_FILE> -s <START_DATE> -e <END_DATE>
```

Date format:

```text
YYYY-MM-DD
```

Example:

```bash
cargo run -- count -l ~/Desktop/focus_log.txt -s 2025-04-19 -e 2025-04-27
cargo run -- count -l ~/Desktop/focus_log.txt -s 2025-04-19 -e 2025-04-27 --task coding
```

### Count One Day

```bash
cargo run -- count-single-day -l <LOG_FILE> -d <DATE>
```

Example:

```bash
cargo run -- count-single-day -l ~/Desktop/focus_log.txt -d 2025-04-26
cargo run -- count-single-day -l ~/Desktop/focus_log.txt -d 2025-04-26 --task coding
```

### Count Precise Time Range

```bash
cargo run -- count-precise -l <LOG_FILE> -s "<START_TIME>" -e "<END_TIME>"
```

Timestamp format:

```text
YYYY-MM-DD HH:MM:SS
```

Example:

```bash
cargo run -- count-precise -l ~/Desktop/focus_log.txt -s "2025-04-19 22:50:00" -e "2025-04-26 13:45:30"
cargo run -- count-precise -l ~/Desktop/focus_log.txt -s "2025-04-19 22:50:00" -e "2025-04-26 13:45:30" --task coding
```

### Count By Task

```bash
cargo run -- count-by-task -l <LOG_FILE> -s <START_DATE> -e <END_DATE>
```

Example:

```bash
cargo run -- count-by-task -l ~/Desktop/focus_log.txt -s 2025-04-19 -e 2025-04-27
```

Sessions without a task label are grouped as `Unlabeled`.

### Generate Work Trend Chart

```bash
cargo run -- plot -l <LOG_FILE> -p <PLOT_PATH> -s <START_DATE> -e <END_DATE>
```

Example:

```bash
cargo run -- plot -l ~/Desktop/focus_log.txt -p ~/Desktop/plot.png -s 2025-04-16 -e 2025-04-29
```

## Web API

The local web server registers these endpoints:

- `POST /rest`
- `POST /rest/pause`
- `POST /rest/resume`
- `POST /rest/stop`
- `GET /rest/status`
- `POST /count`
- `POST /count-by-task`
- `POST /count-single-day`
- `POST /count-precise`
- `POST /plot`
- `POST /log-preview`
- `GET /processes`
- `GET /dialog/directory`
- `GET /dialog/file`
- `GET /dialog/save-file`

The `/dialog/*` endpoints are intended for the local web UI. They open native OS dialogs and are not useful on a remote server.

## Build

Install Rust, then run:

```bash
cargo build --release
```

The binary is created under:

```text
target/release/
```

This project embeds Python through `pyo3` for plugins. If your Python version is newer than what the current `pyo3` release supports, build with:

```bash
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo build --release
```

## Test

```bash
cargo test
```

Optional shell tests:

```bash
cd tests
./test_interactive.sh
./test_plugins.sh
```

On Windows, use:

```powershell
.\test_plugins_windows.ps1
```

## Plugins

Python plugins live under [`plugins/`](plugins/).

Supported hooks:

- `on_init`
- `on_work_start`
- `on_break_reminder`

Generate a plugin template:

```bash
cargo run -- gen -n my_plugin
```

Example plugins are included:

- [`plugins/hello_world.py`](plugins/hello_world.py)
- [`plugins/get_crypto_prices.py`](plugins/get_crypto_prices.py)
- [`plugins/snake_game.py`](plugins/snake_game.py)

## Log Format

New work sessions are written as JSON lines so each entry can store timestamps, duration, monitored apps, and an optional task label:

```json
{"start":"2025-04-19T22:16:15+08:00","end":"2025-04-19T22:46:32+08:00","duration_seconds":1817,"apps":["Cursor"],"task":"coding"}
```

Statistics and plotting commands still support legacy text lines like `[2025-04-19 22:16:15 ~ 2025-04-19 22:46:32] You worked for 30.28 minutes`.
