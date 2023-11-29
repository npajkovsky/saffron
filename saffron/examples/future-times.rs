//! Prints an list of times from a cron iterator until the DateTime container is maxed out

use chrono::Utc;
use saffron::{parse::CronExpr, Cron};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args
        .get(1)
        .map(|s| s.as_str().parse::<CronExpr>())
        .transpose()
    {
        Ok(Some(cron)) => {
            dbg!(&cron);
        }
        Ok(None) => println!("Usage: cargo run --example future-times -- \"[cron expression]\""),
        Err(err) => println!("{}", err),
    }
}
