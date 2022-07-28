extern crate ptime;

mod shams;
mod utils;

use tabled::{Alignment, builder::Builder, Header, Margin, Modify, ModifyObject, Style, Width};
use colored_truecolor::Colorize;
use ptime::Tm;
use tabled::object::{Rows, Segment};
use utils::config::Config;
use shams::calendar;
use crate::calendar::Calendar;
use crate::utils::{constants, storage_helper};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result{
    let mut config = Config::load().await;
    let persian_calendar = ptime::now();

    let mut calendar = Calendar::new(&persian_calendar);
    print_calendar(&mut calendar, config);

    Ok(())
}



fn print_calendar(calendar: &mut Calendar, config: Config) {
    let month_start_point = ptime::from_persian_date(
        calendar.this_year,
        calendar.this_month,
        1
    ).unwrap();

    // let start_offset = 6 - month_start_point.tm_wday;
    let days_count = constants::get_day_count(
        calendar.this_month,
        month_start_point.is_leap()
    );

    calendar.days = Calendar::days(month_start_point.tm_wday, days_count);

    let mut builder = Builder::default();
    builder.set_columns(&calendar.days_titles);

    let mut start = 0;
    let mut until = 7;

    loop {
        let friday = calendar.days.get(until - 1).unwrap().red().to_string();
        calendar.days[until - 1] = friday;

        builder.add_record(&calendar.days[start..until]);

        start = until;
        until = until + 7;

        if until >= calendar.days.len() - 1 {
            break;
        }
    }

    let week_day = if calendar.week_day == 0 {
        "Shanbeh".to_string()
    }else if calendar.week_day == 6 {
        "Adineh".to_string()
    }else {
        format!("{}-Shanbeh", calendar.week_day)
    };
    let formatted_header = format!(
        r"
Emruz: {}  {}  {}({})  {}  {}
        ",
        week_day,
        calendar.today + 1,
        calendar.month_name,
        calendar.this_month,
        calendar.this_year,
        "Tabestan"
    ).bold().to_string();

    let table = builder.build()
        .with(Header(formatted_header))
        .with(Style::extended())
        .with(Segment::all().modify().with(Alignment::center()));
    println!("{}", table);
}