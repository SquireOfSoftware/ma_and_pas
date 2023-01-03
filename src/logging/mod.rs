use chrono::prelude::*;
use log::{LevelFilter, Record};
use std::io::Write;
use std::thread;

use env_logger::fmt::Formatter;
use env_logger::Builder;

pub fn setup_logger() {
    let output_format = move |formatter: &mut Formatter, record: &Record| {
        let thread_name = format!("(t: {})", thread::current().name().unwrap_or("unknown"));
        let local_time: DateTime<Local> = Local::now();
        let time_str = local_time.format("%H:%M:%S%.3f").to_string();
        write!(
            formatter,
            "{} {}{} - {} - {}\n",
            time_str,
            thread_name,
            record.level(),
            record.target(),
            record.args()
        )
    };

    let mut builder = Builder::new();
    builder
        .format(output_format)
        .filter(None, LevelFilter::Info);
    // rust_log.map(|conf| builder.parse_filters(conf));
    builder.init();
}

#[allow(dead_code)]
fn main() {
    println!("Welcome to the logging lib");
}
