use std::error::Error;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::path::PathBuf;
use chrono::{DateTime, Local};

pub fn acc_work_time_precise(log_location: PathBuf, start: DateTime<Local>, end:DateTime<Local>) -> Result<i64, Box<dyn Error>> {
    if end < start {
        panic!("End time must be greater than start time!");
    } else if end == start {
        return Ok(0);
    }
    
    let file = OpenOptions::new()
        .read(true)
        .open(log_location)?;
    let reader = std::io::BufReader::new(file);
    let mut work_time = chrono::Duration::zero();
    
    for line in reader.lines() {
        let line = line?;
        if !line.starts_with('[') {
            continue;
        }
        
        if let Some(end_bracket) = line.find(']') {
            // get "2025-04-19 22:16:15 ~ 2025-04-19 22:16:32" string
            let times = &line[1..end_bracket];
            let mut parts = times.split(" ~ ");
            let start_time = parts.next().unwrap();
            let end_time = parts.next().unwrap();
            
            let start_time = start_time.parse::<DateTime<Local>>()?;
            let end_time = end_time.parse::<DateTime<Local>>()?;
            
            if start_time <= start && end_time <= end {
                work_time += end_time - start_time;
                return Ok(work_time.num_seconds());
            }
            
            if start_time <= start && start <= end_time {
                work_time += end_time - start;
            } else if start <= start_time && end_time <= end {
                work_time += end_time - start_time;
            } else if start_time <= end && end <= end_time {
                work_time += end - start_time;
            }
        }
    }
    
    Ok(work_time.num_seconds())
}

pub fn acc_work_time(log_location: PathBuf, start_day: DateTime<Local>, end_day: DateTime<Local>) -> Result<i64, Box<dyn std::error::Error>> {
    if end_day < start_day {
        panic!("End day must be greater than start day!");
    } else if end_day == start_day {
        return single_day_work_time(log_location, start_day);
    }
    
    let file = OpenOptions::new()
        .read(true)
        .open(log_location)?;
    let reader = std::io::BufReader::new(file);
    let mut work_time = chrono::Duration::zero();
    
    for line in reader.lines() {
        let line = line?;
        if !line.starts_with('[') {
            continue;
        }

        if let Some(end_bracket) = line.find(']') {
            // get "2025-04-19 22:16:15 ~ 2025-04-19 22:16:32" string
            let times = &line[1..end_bracket];
            let mut parts_split_with_space = times.split(" ");
            let s = parts_split_with_space.next().unwrap(); // 2025-04-19
            let e = parts_split_with_space.nth(3).unwrap(); // 2025-04-19
            let start_time = s.parse::<DateTime<Local>>()?;
            let end_time = e.parse::<DateTime<Local>>()?;
            
            let mut parts_split_with_squiggle = s.split(" ~ ");
            let s = parts_split_with_squiggle.next().unwrap(); // 2025-04-19 22:16:15
            let e = parts_split_with_squiggle.next().unwrap(); // 2025-04-19 22:16:32
            let detailed_start_time = s.parse::<DateTime<Local>>()?;
            let detailed_end_time = e.parse::<DateTime<Local>>()?;
            
            if start_time == start_day {
                work_time += detailed_end_time - detailed_start_time;
            } else if start_day < start_time && end_time <= end_day {
                work_time += detailed_end_time - detailed_start_time;
            }
        }
    }
        
    Ok(work_time.num_seconds())
}

// TODO
pub fn single_day_work_time(log_location: PathBuf, day: DateTime<Local>) -> Result<i64, Box<dyn std::error::Error>> {
    Ok(0)
}