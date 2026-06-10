use chrono::{DateTime, Local, LocalResult, NaiveDateTime, TimeZone};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub duration_seconds: i64,
    #[serde(default)]
    pub apps: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
}

impl LogEntry {
    pub fn new(
        start: DateTime<Local>,
        end: DateTime<Local>,
        apps: Vec<String>,
        task: Option<String>,
    ) -> Self {
        let duration_seconds = (end - start).num_seconds();
        Self {
            start,
            end,
            duration_seconds,
            apps,
            task: normalize_task(task),
        }
    }

    pub fn to_json_line(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn task_matches(&self, task_filter: Option<&str>) -> bool {
        let Some(task_filter) = normalize_task(task_filter.map(str::to_string)) else {
            return true;
        };

        self.task
            .as_deref()
            .is_some_and(|task| task.eq_ignore_ascii_case(&task_filter))
    }
}

pub fn parse_log_line(line: &str) -> Result<Option<LogEntry>, Box<dyn Error>> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    if trimmed.starts_with('{') {
        let entry = serde_json::from_str::<LogEntry>(trimmed)?;
        return Ok(Some(entry));
    }

    if trimmed.starts_with('[') {
        return parse_legacy_log_line(trimmed);
    }

    Ok(None)
}

fn parse_legacy_log_line(line: &str) -> Result<Option<LogEntry>, Box<dyn Error>> {
    let Some(end_bracket) = line.find(']') else {
        return Ok(None);
    };

    let times = &line[1..end_bracket];
    let mut parts = times.split(" ~ ");
    let Some(start_time) = parts.next() else {
        return Ok(None);
    };
    let Some(end_time) = parts.next() else {
        return Ok(None);
    };

    let start = parse_local_datetime(start_time)?;
    let end = parse_local_datetime(end_time)?;
    Ok(Some(LogEntry::new(start, end, Vec::new(), None)))
}

fn parse_local_datetime(value: &str) -> Result<DateTime<Local>, Box<dyn Error>> {
    let naive = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")?;
    match Local.from_local_datetime(&naive) {
        LocalResult::Single(dt) => Ok(dt),
        LocalResult::Ambiguous(dt, _) => Ok(dt),
        LocalResult::None => Err("Ambiguous or invalid local time".into()),
    }
}

fn normalize_task(task: Option<String>) -> Option<String> {
    task.map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
