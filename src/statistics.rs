// use std::fs::OpenOptions;
// use std::io::BufRead;
// use std::path::PathBuf;
// use chrono::{DateTime, Local};
// 
// pub fn acc_work_time(log_location: PathBuf, start: DateTime<Local>, end:DateTime<Local>) -> f64 {
//     if end - start <= chrono::Duration::seconds(0) {
//         panic!("End time must be greater than start time!");
//     }
//     
//     let file = OpenOptions::new()
//         .read(true)
//         .open(log_location)
//         .expect("Failed to open focus_log,txt file");
//     
//     let reader = std::io::BufReader::new(file);
//     for line in reader.lines() {
//         let line = line.unwrap();
//         let line_split: Vec<&str> = line.split('\r').collect();
//         
//         // TODO
//         let time_str = &line_split[1..19];
//         let time: DateTime<Local> = time_str.parse().unwrap_or_else(|e| { panic!("Failed to parse time: {}", e) });
//         
//         if time - start < chrono::Duration::seconds(0) {
//             continue;
//         } 
//         
//         
//         
//     }
//     
//     0.0
// }