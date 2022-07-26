extern crate ptime;

mod shams;
mod utils;

use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use colored_truecolor::Colorize;
use utils::config::Config;
use utils::server;
use utils::config;
use shams::calendar;
use crate::calendar::Calendar;
use crate::shams::occasions::{Occasion, Occasions};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result{
    let config = Config::load();
    let persian_calendar = ptime::now();

    if !config.is_occasions_fetched {
        check_internet_for_occasions(persian_calendar.tm_year as u16).await;
    }else if config.fetched_occasions_year.is_some() {
        if config.fetched_occasions_year.unwrap() != persian_calendar.tm_year as u16 {
            check_internet_for_occasions(persian_calendar.tm_year as u16).await;
        }
    }

    Ok(())
}

async fn check_internet_for_occasions(year: u16) {
    let mut occasions: Option<Vec<Occasions>> = None;
    let online = online::check(Some(3)).await;
    if online.is_ok() {
        occasions = server::fetch_occasions(year).await;
        handle_occasions(occasions);
    } else {
        println!("{}", "Internet is not available for fetch Occasions from server!".red());
    }
}

fn handle_occasions(_occasions: Option<Vec<Occasions>>) {
    if let Some(occasions) = _occasions {
        //TODO handle this
    }
}