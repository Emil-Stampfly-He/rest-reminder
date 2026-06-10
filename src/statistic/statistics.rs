use crate::statistic::log_entry::{LogEntry, parse_log_line};
use chrono::{DateTime, Duration, Local, LocalResult, TimeZone};
use serde::Serialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub const UNLABELED_TASK: &str = "Unlabeled";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskWorkSummary {
    pub task: String,
    pub seconds: i64,
}

pub fn acc_work_time_precise(
    log_location: PathBuf,
    start: DateTime<Local>,
    end: DateTime<Local>,
) -> Result<i64, Box<dyn Error>> {
    acc_work_time_precise_for_task(log_location, start, end, None)
}

pub fn acc_work_time_precise_for_task(
    log_location: PathBuf,
    start: DateTime<Local>,
    end: DateTime<Local>,
    task: Option<&str>,
) -> Result<i64, Box<dyn Error>> {
    if end < start {
        panic!("End time must be greater than start time!");
    } else if end == start {
        return Ok(0);
    }

    let entries = parse_log_entries(&log_location)?;
    Ok(calculate_overlap(&entries, start, end, task))
}

pub fn acc_work_time(
    log_location: PathBuf,
    start_day: DateTime<Local>,
    end_day: DateTime<Local>,
) -> Result<i64, Box<dyn Error>> {
    acc_work_time_for_task(log_location, start_day, end_day, None)
}

pub fn acc_work_time_for_task(
    log_location: PathBuf,
    start_day: DateTime<Local>,
    end_day: DateTime<Local>,
    task: Option<&str>,
) -> Result<i64, Box<dyn Error>> {
    if end_day < start_day {
        panic!("End day must be greater than start day!");
    } else if end_day == start_day {
        return single_day_work_time_for_task(log_location, start_day, task);
    }

    // To NaiveDateTime
    let naive_start = start_day.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let naive_end = end_day.date_naive().and_hms_opt(23, 59, 59).unwrap();

    // To DateTime<Local>
    let start_dt = match Local.from_local_datetime(&naive_start) {
        LocalResult::Single(dt) => dt,
        _ => panic!("Invalid local start_day"),
    };
    let end_dt = match Local.from_local_datetime(&naive_end) {
        LocalResult::Single(dt) => dt,
        _ => panic!("Invalid local end_day"),
    };

    let entries = parse_log_entries(&log_location)?;
    Ok(calculate_overlap(&entries, start_dt, end_dt, task))
}

pub fn task_work_time_summary(
    log_location: PathBuf,
    start_day: DateTime<Local>,
    end_day: DateTime<Local>,
) -> Result<Vec<TaskWorkSummary>, Box<dyn Error>> {
    if end_day < start_day {
        panic!("End day must be greater than start day!");
    }

    let naive_start = start_day.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let naive_end = end_day.date_naive().and_hms_opt(23, 59, 59).unwrap();
    let start_dt = match Local.from_local_datetime(&naive_start) {
        LocalResult::Single(dt) => dt,
        _ => panic!("Invalid local start_day"),
    };
    let end_dt = match Local.from_local_datetime(&naive_end) {
        LocalResult::Single(dt) => dt,
        _ => panic!("Invalid local end_day"),
    };

    let entries = parse_log_entries(&log_location)?;
    let mut summary = BTreeMap::new();
    for entry in entries {
        let overlap_seconds = overlap_seconds(&entry, start_dt, end_dt);
        if overlap_seconds <= 0 {
            continue;
        }

        let task = entry
            .task
            .clone()
            .unwrap_or_else(|| UNLABELED_TASK.to_string());
        *summary.entry(task).or_insert(0) += overlap_seconds;
    }

    Ok(summary
        .into_iter()
        .map(|(task, seconds)| TaskWorkSummary { task, seconds })
        .collect())
}

pub fn single_day_work_time(
    log_location: PathBuf,
    day: DateTime<Local>,
) -> Result<i64, Box<dyn Error>> {
    single_day_work_time_for_task(log_location, day, None)
}

pub fn single_day_work_time_for_task(
    log_location: PathBuf,
    day: DateTime<Local>,
    task: Option<&str>,
) -> Result<i64, Box<dyn Error>> {
    let naive_start = day.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let naive_end = day.date_naive().and_hms_opt(23, 59, 59).unwrap();

    // To DateTime<Local>
    let start_dt = match Local.from_local_datetime(&naive_start) {
        LocalResult::Single(dt) => dt,
        _ => return Err("Invalid local start time".into()),
    };
    let end_dt = match Local.from_local_datetime(&naive_end) {
        LocalResult::Single(dt) => dt,
        _ => return Err("Invalid local end time".into()),
    };

    let entries = parse_log_entries(&log_location)?;
    Ok(calculate_overlap(&entries, start_dt, end_dt, task))
}

fn parse_log_entries(log_location: &PathBuf) -> Result<Vec<LogEntry>, Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open(log_location)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(entry) = parse_log_line(&line)? {
            entries.push(entry);
        }
    }

    Ok(entries)
}

fn calculate_overlap(
    entries: &[LogEntry],
    range_start: DateTime<Local>,
    range_end: DateTime<Local>,
    task: Option<&str>,
) -> i64 {
    entries
        .iter()
        .fold(Duration::zero(), |acc, entry| {
            if !entry.task_matches(task) {
                return acc;
            }

            let seconds = overlap_seconds(entry, range_start, range_end);
            if seconds > 0 {
                acc + Duration::seconds(seconds)
            } else {
                acc
            }
        })
        .num_seconds()
}

fn overlap_seconds(
    entry: &LogEntry,
    range_start: DateTime<Local>,
    range_end: DateTime<Local>,
) -> i64 {
    let overlap_start = entry.start.max(range_start);
    let overlap_end = entry.end.min(range_end);
    if overlap_start < overlap_end {
        (overlap_end - overlap_start).num_seconds()
    } else {
        0
    }
}
