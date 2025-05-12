# Rest Reminder (Windows User Only Currently)

![Lines of Code](https://img.shields.io/endpoint?url=https://Emil-Stampfly-He.github.io/rest-reminder/badge.json)

简体中文：[README_zh.md](./README-zh_CN.md).

A small Rust-based Windows application that monitors whether you are working, and reminds you to take a break after specified time of continuous usage.
It also logs your focused work sessions to a file, helps you calculate your total work time and generate your working time trend chart.

## Features

- Monitors specific processes (e.g., `idea64.exe`, `rustrover64.exe`)
- Tracks focused work time
- Displays a blocking system popup message after indicated continuous work time
- Displays your working trend

## Working Sample

> ![Screenshot](Screenshot.png)
> ![Working Trend](example.png)

### Sample `focus_log.txt`: [`focus_log.txt`](focus_log.txt)

## How It Works

1. The program checks if specified process(es) are currently running.
2. If they are detected, it starts tracking the time.
3. If specified time passes without closing the process(es), a system popup appears to remind you to take a break.
4. The session is logged to a file for record-keeping.
5. Use `count` family commands to calculate your total work time in a certain period.
6. Use `plot` commands to show your working trend.

## How to Use
As described above, there are three different ways to use your Rest Reminder.

### 1. Detecting your work

To start counting your work time, run:
```aiignore
rest-reminder.exe rest -- -l <PATH> -t <TIME> -a <PROCESS_1> <PROCESS_2> ...
```
* If you do not indicate your `focus_log.txt` file location, it will be set to be `D:\` in default
* For blank `<TIME>`, you need to indicate in second, not minute or hour. The default work time is set to 3600 seconds (1 hour)
* You can also indicate all the processes you would like you Rest Reminder to detect. The default processes are `idea64.exe` for IntelliJ IDEA and `rustrover64.exe` for RustRover

For example:
```aiignore
rest-reminder.exe rest -- -l D:\ -t 3600 -a Notion.exe Code.exe
```
* By indicate `D:\`, you are saving your `focus_log.txt` under your `D:\` directory
* By indicating `3600`, you are telling your Rest Reminder to remind you to relax every 1 hour
* By indicating `Notion.exe` and `Code.exe`, you are requiring Rest Reminder to detect if **Notion** or **VS Code** is working

To see the name of a process, open your **Task Manager**.

> **REMINDER: DO NOT add `focus_log.txt` after your file location!** 
> 
> For example:
> * `D:\`: allowed
> * `D:\focus_log.txt`: **NOT** allowed
> * `D:\name`: **NOT** allowed, since you missed a `\` suffix

### 2. Calculate your accumulated work time
There are three possible ways to calculate your work time:
1. `count`
2. `count-single-day`
3. `count-precise`

#### 2.1. Count your work time daily basis
If you would like to specify an exact time interval, run the following:
```aiignore
rest.reminder.exe count -- -l <PATH> -s <START> -e <END>
```
* `PATH`: indicate the full file location of your `focus_log.txt`
* `START` and `END`: follow `YYYY-MM-DD` format

For example:
```aiignore
rest-reminder.exe count -- -l D:\focus_log.txt -s 2025-04-19 -e 2025-04-27
```
Then, the Rest Reminder will automatically count your total working time during this period. 

> **ATTENTION**: **DO NOT** forget to bring `\focus_log.txt` at the end of your <PATH> variable.

#### 2.2 Count your one-day work time
To know how long you worked on an exact date, run the following:
```aiignore
rest-reminder.exe count-single-day -- -l <PATH> -d <DAY>
```
Use `YYYY-MM-DD` format for `PATH` variable as above. For example:
```aiignore
rest-reminder.exe count-single-day -- -l D:\focus_log.txt -d 2025-04-26
```
Then it will help to calculate your total work time in 2025-04-26.

#### 2.3 Count your precise work time
Sometimes you do with to know exactly how long you worked for a certain period of time. You can run the following:
```aiignore
rest-reminder.exe count-precise -- -l <PATH> -s <START> -e <END>
```
You need to format your `START` and `END` like `YYYY-MM-DD HH-MM-SS` **AND QUOTE THEM WITH QUOTATION MARKS**. For example:
```aiignore
rest-reminder.exe count-precise -- -l D:\focus_log.txt -s "2025-04-19 22:50:00" -e "2025-04-26 13:45:30"
```

### 3. Plot your working trend
You can also have an insight into your working trend with Rest Reminder! Run following:
```aiignore
rest-reminder.exe plot -- -l <LOG_PATH> -p <PLOT_PATH> -s <START> -e <END>
```
* `<LOG-PATH>`: location of your `focus_log.txt` file
* `<PLOT-PATH>`: location of your working trend chart to save to
* `<START>` & `<END>`: follow the format `YYYY-MM-DD`

For example:
```aiignore
rest-reminder.exe plot -- -l D:\focus_log.txt -p D:\plot.png -s 2025-04-16 -e 2025-04-29
```
Rest Reminder will generate your working trend during 2025-04-16 to 2025-04-29 and save your `plot.png` picture under `D:\`.
> **ATTENTION**: you need to name your picture like `plot.png` above!

