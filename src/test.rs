#[cfg(test)]
mod tests {
    use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
    use std::path::PathBuf;
    use crate::statistics::acc_work_time_precise;

    fn local_dt(s: &str) -> DateTime<Local> {
        let naive = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
            .expect("Format should be %Y-%m-%d %H:%M:%S");
        Local.from_local_datetime(&naive)
            .single()
            .expect("Should be a valid date")
    }

    #[test]
    fn test_zero_duration() {
        let path = PathBuf::from(r"D:\focus_log.txt");
        let dt = local_dt("2025-04-19 00:00:00");
        assert_eq!(acc_work_time_precise(path.clone(), dt, dt).unwrap(), 0);
    }

    #[test]
    #[should_panic(expected = "End time must be greater than start time!")]
    fn test_end_before_start() {
        let path = PathBuf::from(r"D:\focus_log.txt");
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
        let path = PathBuf::from(r"D:\focus_log.txt");
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
        let path = PathBuf::from(r"D:\focus_log.txt");
        let start = local_dt("2025-04-19 22:54:50");
        let end = local_dt("2025-04-19 22:55:00");
        assert_eq!(acc_work_time_precise(path, start, end).unwrap(), 10);
    }

    // No record, should be 0
    #[test]
    fn test_no_entries_in_range() {
        let path = PathBuf::from(r"D:\focus_log.txt");
        let start = local_dt("2025-04-19 21:00:00");
        let end = local_dt("2025-04-19 22:00:00");
        assert_eq!(acc_work_time_precise(path, start, end).unwrap(), 0);
    }
}
