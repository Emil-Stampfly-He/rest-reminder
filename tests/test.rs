#[cfg(test)]
mod test_first {
    use chrono::{DateTime, Local, LocalResult, NaiveDateTime, TimeZone};
    use rest_reminder::statistic::statistics::{
        UNLABELED_TASK, acc_work_time, acc_work_time_for_task, acc_work_time_precise,
        acc_work_time_precise_for_task, single_day_work_time, single_day_work_time_for_task,
        task_work_time_summary,
    };
    use std::path::PathBuf;

    const TEST_FOCUS_LOG_PATH: &str = "tests/test_focus_log.txt";

    #[test]
    fn test_zero_duration() {
        let path = PathBuf::from(TEST_FOCUS_LOG_PATH);
        let dt = local_dt("2025-04-19 00:00:00");
        assert_eq!(acc_work_time_precise(path.clone(), dt, dt).unwrap(), 0);
    }

    #[test]
    #[should_panic(expected = "End time must be greater than start time!")]
    fn test_end_before_start() {
        let path = PathBuf::from(TEST_FOCUS_LOG_PATH);
        let start = local_dt("2025-04-19 23:00:00");
        let end = local_dt("2025-04-19 22:00:00");
        let _ = acc_work_time_precise(path, start, end);
    }

    // From 2025-04-19 22:00:00 to 2025-04-19 23:00:00, three records：
    //    [22:48:24 ~ 22:48:37] = 13s
    //    [22:54:44 ~ 22:54:56] = 12s
    //    [22:54:56 ~ 22:55:07] = 11s
    //    total = 13 + 12 + 11 = 36s
    #[test]
    fn test_accumulate_within_range() {
        let path = PathBuf::from(TEST_FOCUS_LOG_PATH);
        let start = local_dt("2025-04-19 22:00:00");
        let end = local_dt("2025-04-19 23:00:00");
        assert_eq!(acc_work_time_precise(path, start, end).unwrap(), 36);
    }

    // Partially overlapping：interval [22:54:50 ~ 22:55:00]
    //    and [22:54:44~22:54:56] has an overlapping band of 6s,
    //    and [22:54:56~22:55:07] has an overlapping band of 4s,
    //    total = 10s
    #[test]
    fn test_partial_overlap_start_inside() {
        let path = PathBuf::from(TEST_FOCUS_LOG_PATH);
        let start = local_dt("2025-04-19 22:54:50");
        let end = local_dt("2025-04-19 22:55:00");
        assert_eq!(acc_work_time_precise(path, start, end).unwrap(), 10);
    }

    // No record, should be 0
    #[test]
    fn test_no_entries_in_range() {
        let path = PathBuf::from(TEST_FOCUS_LOG_PATH);
        let start = local_dt("2025-04-19 21:00:00");
        let end = local_dt("2025-04-19 22:00:00");
        assert_eq!(acc_work_time_precise(path, start, end).unwrap(), 0);
    }

    #[test]
    fn test_single_day_no_entries() {
        let path = PathBuf::from(TEST_FOCUS_LOG_PATH);
        let day = local_date("2025-04-18");
        assert_eq!(single_day_work_time(path, day).unwrap(), 0);
    }

    #[test]
    fn test_single_day_2025_04_19() {
        let path = PathBuf::from(TEST_FOCUS_LOG_PATH);
        let day = local_date("2025-04-19");
        // Entries:
        // 22:48:24–22:48:37 = 13s
        // 22:54:44–22:54:56 = 12s
        // 22:54:56–22:55:07 = 11s
        // 23:15:20–23:15:32 = 12s
        // 23:18:21–23:19:30 = 69s
        // Total = 13 + 12 + 11 + 12 + 69 = 117s
        assert_eq!(single_day_work_time(path, day).unwrap(), 117);
    }

    #[test]
    fn test_single_day_2025_04_21_cross_midnight() {
        let path = PathBuf::from(TEST_FOCUS_LOG_PATH);
        let day = local_date("2025-04-21");
        // Entries on 4/21, including a segment past midnight (counted only until 23:59:59).
        // Verified total seconds: 29,372
        assert_eq!(single_day_work_time(path, day).unwrap(), 29372);
    }

    #[test]
    #[should_panic(expected = "End day must be greater than start day!")]
    fn test_acc_work_time_panic_on_invalid_range() {
        let path = PathBuf::from(TEST_FOCUS_LOG_PATH);
        let start = local_date("2025-04-20");
        let end = local_date("2025-04-19");
        let _ = acc_work_time(path, start, end);
    }

    #[test]
    fn test_acc_work_time_same_day() {
        let path = PathBuf::from(TEST_FOCUS_LOG_PATH);
        let day = local_date("2025-04-20");
        let single = single_day_work_time(path.clone(), day).unwrap();
        let acc = acc_work_time(path, day, day).unwrap();
        assert_eq!(acc, single);
    }

    #[test]
    fn test_acc_work_time_span_19_to_23() {
        let path = PathBuf::from(TEST_FOCUS_LOG_PATH);
        let start = local_date("2025-04-19");
        let end = local_date("2025-04-23");
        assert_eq!(acc_work_time(path, start, end).unwrap(), 129_745);
    }

    #[test]
    fn test_acc_work_time_span_with_empty_days() {
        let path = PathBuf::from(TEST_FOCUS_LOG_PATH);
        let start = local_date("2025-04-18"); // no entries on 4/18
        let end = local_date("2025-04-20");
        // Equivalent to 4/19 (117s) + 4/20 (8063s) = 8180s
        assert_eq!(acc_work_time(path, start, end).unwrap(), 8_180);
    }

    #[test]
    fn test_acc_work_time_precise_handles_cross_day_window_without_overlap() {
        let path = PathBuf::from(TEST_FOCUS_LOG_PATH);
        let start = local_dt("2025-04-21 23:59:50");
        let end = local_dt("2025-04-22 00:00:10");
        assert_eq!(acc_work_time_precise(path, start, end).unwrap(), 0);
    }

    #[test]
    fn test_missing_log_file_returns_error() {
        let path = PathBuf::from("tests/does_not_exist_focus_log.txt");
        let start = local_dt("2025-04-19 22:00:00");
        let end = local_dt("2025-04-19 23:00:00");

        assert!(acc_work_time_precise(path.clone(), start, end).is_err());
        assert!(single_day_work_time(path.clone(), local_date("2025-04-19")).is_err());
        assert!(acc_work_time(path, local_date("2025-04-19"), local_date("2025-04-20")).is_err());
    }

    #[test]
    fn test_log_lines_without_entry_prefix_are_ignored() {
        let path = write_temp_log(&[
            "not a focus log entry",
            "[2025-04-19 10:00:00 ~ 2025-04-19 10:05:00] You worked for 5.00 minutes",
            "another ignored line",
        ]);
        let start = local_dt("2025-04-19 09:00:00");
        let end = local_dt("2025-04-19 11:00:00");

        assert_eq!(acc_work_time_precise(path, start, end).unwrap(), 300);
    }

    #[test]
    fn test_structured_json_log_entries_are_counted() {
        let path = write_temp_log(&[
            &json_log("2025-04-19 10:00:00", "2025-04-19 10:05:00", "coding"),
            &json_log("2025-04-19 10:10:00", "2025-04-19 10:15:00", "review"),
        ]);
        let start = local_dt("2025-04-19 09:00:00");
        let end = local_dt("2025-04-19 11:00:00");

        assert_eq!(acc_work_time_precise(path, start, end).unwrap(), 600);
    }

    #[test]
    fn test_task_filter_only_counts_matching_structured_entries() {
        let path = write_temp_log(&[
            &json_log("2025-04-19 10:00:00", "2025-04-19 10:05:00", "coding"),
            &json_log("2025-04-19 10:10:00", "2025-04-19 10:15:00", "review"),
            "[2025-04-19 10:20:00 ~ 2025-04-19 10:25:00] You worked for 5.00 minutes",
        ]);
        let start = local_dt("2025-04-19 09:00:00");
        let end = local_dt("2025-04-19 11:00:00");
        let day = local_date("2025-04-19");

        assert_eq!(
            acc_work_time_precise_for_task(path.clone(), start, end, Some("coding")).unwrap(),
            300
        );
        assert_eq!(
            single_day_work_time_for_task(path.clone(), day, Some("review")).unwrap(),
            300
        );
        assert_eq!(
            acc_work_time_for_task(path, day, day, Some("missing")).unwrap(),
            0
        );
    }

    #[test]
    fn test_task_summary_groups_labeled_and_legacy_entries() {
        let path = write_temp_log(&[
            &json_log("2025-04-19 10:00:00", "2025-04-19 10:05:00", "coding"),
            &json_log("2025-04-19 10:10:00", "2025-04-19 10:15:00", "review"),
            &json_log("2025-04-19 10:20:00", "2025-04-19 10:23:00", "coding"),
            "[2025-04-19 10:30:00 ~ 2025-04-19 10:32:00] You worked for 2.00 minutes",
        ]);

        let summary =
            task_work_time_summary(path, local_date("2025-04-19"), local_date("2025-04-19"))
                .unwrap();

        assert_eq!(summary.len(), 3);
        assert_eq!(summary[0].task, "Unlabeled");
        assert_eq!(summary[0].seconds, 120);
        assert_eq!(summary[1].task, "coding");
        assert_eq!(summary[1].seconds, 480);
        assert_eq!(summary[2].task, "review");
        assert_eq!(summary[2].seconds, 300);
        assert_eq!(UNLABELED_TASK, "Unlabeled");
    }

    fn local_date(date_str: &str) -> DateTime<Local> {
        let datetime_str = format!("{} 00:00:00", date_str);
        let naive = NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
            .expect("Failed to parse date string");
        match Local.from_local_datetime(&naive) {
            LocalResult::Single(dt) => dt,
            _ => panic!("Ambiguous or invalid local time"),
        }
    }

    fn local_dt(s: &str) -> DateTime<Local> {
        let naive = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
            .expect("Format should be %Y-%m-%d %H:%M:%S");
        Local
            .from_local_datetime(&naive)
            .single()
            .expect("Should be a valid date")
    }

    fn write_temp_log(lines: &[&str]) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "rest_reminder_test_{}_{}.txt",
            std::process::id(),
            Local::now().timestamp_nanos_opt().unwrap()
        ));
        std::fs::write(&path, lines.join("\n")).expect("test log should be writable");
        path
    }

    fn json_log(start: &str, end: &str, task: &str) -> String {
        let start = local_dt(start);
        let end = local_dt(end);
        serde_json::json!({
            "start": start,
            "end": end,
            "duration_seconds": (end - start).num_seconds(),
            "apps": ["Cursor"],
            "task": task,
        })
        .to_string()
    }
}
