use std::thread;
use std::time::{SystemTime, Duration};
use notify_rust::{Notification, Timeout};

use clap::{Parser};

#[derive(Parser)]
struct Opts {
    #[clap(required = true)]
    total_hours: u64,
    #[clap(required = true)]
    work_mins: u64,
    #[clap(required = true)]
    break_mins: u64,
}

fn main() {

    let opts: Opts = Opts::parse();

    let total_duration_hours = opts.total_hours;
    let work_mins = opts.work_mins;
    let break_mins = opts.break_mins;

    let start = SystemTime::now();
    let end = start + Duration::from_secs(total_duration_hours * 60 * 60);

    let work_duration = Duration::from_secs(work_mins * 60);
    let break_duration = Duration::from_secs(break_mins * 60);

    while start != end {
        
        let mins_left = end.duration_since(start).unwrap().as_secs() / 60;
        let body_text = format!("for the next {} mins ({} mins left)", work_mins, mins_left);

        Notification::new()
            .summary("Time to work")
            .body(
                &body_text
            )
            .timeout(Timeout::Milliseconds(10_000))
            .show().unwrap();
        println!("[{:?}] Time to work {}", SystemTime::now(), body_text);
        thread::sleep(work_duration);

        let mins_left = end.duration_since(start).unwrap().as_secs() / 60;
        let body_text = format!("come back in {} mins ({})", break_mins, mins_left);

        Notification::new()
            .summary("Take a break,")
            .body(
                &body_text
            )
            .timeout(Timeout::Milliseconds(10_000))
            .show().unwrap();
        println!("[{:?}] Take a break, {}", SystemTime::now(), body_text);
        thread::sleep(break_duration);
    }
}
