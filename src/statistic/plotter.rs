use std::path::PathBuf;
use chrono::{DateTime, Local};
use plotters::chart::ChartBuilder;
use plotters::prelude::*;
use crate::statistic::statistics::single_day_work_time;

pub fn plot(
    log_location: PathBuf,
    plot_location: PathBuf,
    start_day: DateTime<Local>,
    end_day: DateTime<Local>
) -> Result<(), Box<dyn std::error::Error>> {
    let mut date = start_day;
    let mut dots: Vec<(DateTime<Local>, f64)> = Vec::new();

    while date <= end_day {
        let daily_work_time = single_day_work_time(log_location.clone(), date)
            .expect("Failed to get work time");
        dots.push((date, daily_work_time as f64 / 60.0));
        date = date + chrono::Duration::days(1);
    }

    plot_helper(dots, plot_location, start_day, end_day)?;
    Ok(())
}

fn plot_helper(
    dots: Vec<(DateTime<Local>, f64)>,
    plot_location: PathBuf,
    start_day: DateTime<Local>,
    end_day: DateTime<Local>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (min_y, max_y) = dots
        .iter()
        .map(|&(_, v)| v)
        .fold((f64::MAX, f64::MIN), |(min, max), v| {
            (min.min(v), max.max(v))
        });
    let pad = (((max_y - min_y) as f32) * 0.1).ceil() as f64;
    let y_range = (min_y as i64).saturating_sub(pad as i64) as f64..(max_y + pad);
    
    let root = BitMapBackend::new(&plot_location, (1280, 780))
        .into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(40, 40, 40, 40);
    
    let caption = format!(
        "Your Working Trend: {} to {}",
        start_day.format("%Y-%m-%d"),
        end_day.format("%Y-%m-%d")
    );
    let font = ("sans-serif", 32).into_font().style(FontStyle::Bold);
    
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, font)
        .x_label_area_size(100)
        .y_label_area_size(100)
        .build_cartesian_2d(start_day..end_day, y_range)?;
    
    chart.configure_mesh()
        .x_labels(6)
        .y_labels(5)
        .x_label_formatter(&|dt| dt.format("%b %d").to_string())
        .x_desc("Date")
        .y_desc("Work Time (minutes)")
        .x_label_style(("sans-serif", 20).into_font())
        .y_label_style(("sans-serif", 20).into_font())
        .axis_desc_style(("sans-serif", 24).into_font())
        .y_label_formatter(&|y| format!("{} min", y))
        .axis_style(&BLACK.mix(0.8))
        .draw()?;
    
    let line_series = LineSeries::new(
        dots.iter().map(|(dt, v)| (*dt, *v)),
        &BLUE.mix(0.7),
    );
    chart.draw_series(line_series)?;
    
    chart.draw_series(
        dots.iter().map(|(dt, v)| {
            Circle::new((*dt, *v), 4, BLUE.mix(0.7).filled())
        }),
    )?;
    
    root.present()?;
    Ok(())
}



