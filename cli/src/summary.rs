use crate::{
    format_duration,
    timesheet::{Tag, Timesheet},
};
use chrono::{DateTime, Local};
use std::collections::HashSet;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct SummaryCmd {
    /// A list of tags to filter against
    tags: Vec<String>,

    #[structopt(long = "start")]
    start: Option<DateTime<Local>>,
}

impl Default for SummaryCmd {
    fn default() -> Self {
        Self {
            tags: Vec::new(),
            start: None,
        }
    }
}

impl SummaryCmd {
    pub fn exec(&self, timesheet: &Timesheet) {
        let tags: HashSet<Tag> = self.tags.iter().map(|s| Tag(s.clone())).collect();

        let start = self.start.unwrap_or(default_start());
        let segments = timesheet
            .segments()
            .into_iter()
            .filter(|s| s.start_time.with_timezone(&Local) >= start)
            .filter(|s| s.tags.is_superset(&tags));

        let mut total_duration = chrono::Duration::seconds(0);
        let mut current_date = None;

        println!("Date  Start Duration Total     Tags");
        println!(
            "――――― ――――― ―――――――― ――――――――  ――――――――"
        );
        for segment in segments {
            let date = segment.start_time.date();
            let date_str = if current_date != Some(date) {
                current_date = Some(date);
                segment
                    .start_time
                    .date()
                    .with_timezone(&chrono::Local)
                    .format("%m/%d")
                    .to_string()

            } else {
                String::from("     ")
            };
            let start_time = segment
                .start_time
                .with_timezone(&chrono::Local)
                .format("%H:%M");
            let tags_str = segment
                .tags
                .iter()
                .fold(String::new(), |acc, x| format!("{} {}", acc, x.0));

            total_duration = total_duration + segment.duration;

            let duration_str = format_duration(segment.duration);
            let total_duration_str = format_duration(total_duration);

            println!(
                "{} {} {: <8} {: <8} {}",
                date_str, start_time, duration_str, total_duration_str, tags_str
            );
        }
    }
}

fn default_start() -> DateTime<Local> {
    Local::today().and_hms(0, 0, 0)
}