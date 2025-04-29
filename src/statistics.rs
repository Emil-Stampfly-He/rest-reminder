use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use chrono::{DateTime, Duration, Local, LocalResult, NaiveDateTime, TimeZone};

pub fn acc_work_time_precise(
    log_location: PathBuf,
    start: DateTime<Local>,
    end:DateTime<Local>
) -> Result<i64, Box<dyn Error>> {
    if end < start {
        panic!("End time must be greater than start time!");
    } else if end == start {
        return Ok(0);
    }
    
    let file = OpenOptions::new()
        .read(true)
        .open(log_location)?;
    let reader = BufReader::new(file);
    let mut work_time = Duration::zero();
    
    for line in reader.lines() {
        let line = line?;
        if !line.starts_with('[') {
            continue;
        }
        
        if let Some(end_bracket) = line.find(']') {
            // Get "2025-04-19 22:16:15 ~ 2025-04-19 22:16:32" string
            let times = &line[1..end_bracket];
            let mut parts = times.split(" ~ ");
            let start_time = parts.next().unwrap();
            let end_time = parts.next().unwrap();

            let start_naive = NaiveDateTime::parse_from_str(start_time,"%Y-%m-%d %H:%M:%S")?;
            let end_naive = NaiveDateTime::parse_from_str(end_time,"%Y-%m-%d %H:%M:%S")?;
            let log_start_time = Local
                .from_local_datetime(&start_naive)
                .single()
                .ok_or("Ambiguous or invalid local time")?;
            let log_end_time = Local
                .from_local_datetime(&end_naive)
                .single()
                .ok_or("Ambiguous or invalid local time")?;

            let overlap_start = log_start_time.max(start);
            let overlap_end = log_end_time.min(end);
            if overlap_start < overlap_end {
                work_time += overlap_end - overlap_start;
            }
        }
    }
    
    Ok(work_time.num_seconds())
}

pub fn acc_work_time(
    log_location: PathBuf,
    start_day: DateTime<Local>,
    end_day: DateTime<Local>
) -> Result<i64, Box<dyn Error>> {
    if end_day < start_day {
        panic!("End day must be greater than start day!");
    } else if end_day == start_day {
        return single_day_work_time(log_location, start_day);
    }

    let naive_start = start_day.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let naive_end   = end_day  .date_naive().and_hms_opt(23,59,59).unwrap();
    let start_dt = match Local.from_local_datetime(&naive_start) {
        LocalResult::Single(dt) => dt,
        _ => panic!("Invalid local start_day"),
    };
    let end_dt = match Local.from_local_datetime(&naive_end) {
        LocalResult::Single(dt) => dt,
        _ => panic!("Invalid local end_day"),
    };

    let entries = parse_log_entries(&log_location)?;
    Ok(calculate_overlap(&entries, start_dt, end_dt))
}

pub fn single_day_work_time(
    log_location: PathBuf,
    day: DateTime<Local>
) -> Result<i64, Box<dyn Error>> {
    let naive_start = day.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let naive_end   = day.date_naive().and_hms_opt(23, 59, 59).unwrap();
    let start_dt = match Local.from_local_datetime(&naive_start) {
        LocalResult::Single(dt) => dt,
        _ => return Err("Invalid local start time".into()),
    };
    let end_dt = match Local.from_local_datetime(&naive_end) {
        LocalResult::Single(dt) => dt,
        _ => return Err("Invalid local end time".into()),
    };

    let entries = parse_log_entries(&log_location)?;
    Ok(calculate_overlap(&entries, start_dt, end_dt))
}

fn parse_log_entries(log_location: &PathBuf) -> Result<Vec<(DateTime<Local>, DateTime<Local>)>, Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open(log_location)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if !line.starts_with('[') { continue; }
        let end_bracket = match line.find(']') {
            Some(i) => i,
            None => continue,
        };
        let times_str = &line[1..end_bracket]; // "YYYY-MM-DD HH:MM:SS ~ YYYY-MM-DD HH:MM:SS"
        let mut parts  = times_str.split(" ~ ");
        let s_str = parts.next().unwrap();
        let e_str = parts.next().unwrap();
        
        let s_naive = NaiveDateTime::parse_from_str(s_str, "%Y-%m-%d %H:%M:%S")?;
        let e_naive = NaiveDateTime::parse_from_str(e_str, "%Y-%m-%d %H:%M:%S")?;

        // To DateTime<Local>
        let log_start = match Local.from_local_datetime(&s_naive) {
            LocalResult::Single(dt) => dt,
            _ => continue,
        };
        let log_end   = match Local.from_local_datetime(&e_naive) {
            LocalResult::Single(dt) => dt,
            _ => continue,
        };

        entries.push((log_start, log_end));
    }

    Ok(entries)
}

fn calculate_overlap(
    entries: &[(DateTime<Local>, DateTime<Local>)],
    range_start: DateTime<Local>,
    range_end:   DateTime<Local>,
) -> i64 {
    entries.iter().fold(Duration::zero(), |acc, &(s, e)| {
        let overlap_start = s.max(range_start);
        let overlap_end   = e.min(range_end);
        if overlap_start < overlap_end {
            acc + (overlap_end - overlap_start)
        } else {
            acc
        }
    }).num_seconds()
}