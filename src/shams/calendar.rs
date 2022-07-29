use ptime::Tm;
use tabled::{Alignment, builder::Builder, Header, ModifyObject, Style};
use tabled::object::Segment;
use colored_truecolor::Colorize;
use crate::shams::constants;

#[derive(Debug, Clone)]
pub struct Calendar {
    pub this_year: i32,
    pub this_month: i32,
    pub week_day: i32,
    pub today: i32,
    pub month_name: String,
    pub days_titles: [String; 7],
    pub days: Vec<String>
}

impl Calendar {

    pub fn new(pt: &Tm) -> Self {
        let days = [
            " Shanbe ".bold().green().to_string(),
            "   Yek  ".bold().green().to_string(),
            "   Do   ".bold().green().to_string(),
            "   Se   ".bold().green().to_string(),
            " Chahar ".bold().green().to_string(),
            "  Panj  ".bold().green().to_string(),
            " Adineh ".bold().green().to_string(),
        ];

        let this_month = pt.tm_mon;
        let calendar = Calendar {
            this_year: pt.tm_year,
            this_month,
            week_day: pt.tm_wday,
            today: pt.tm_mday,
            month_name: constants::get_month_name(this_month),
            days_titles: days,
            days: vec![]
        };
        calendar
    }

    pub fn days(start_offset: i32, days_count: i32, today: i32) -> Vec<String> {
        let mut numeric_days = vec![];
        for i in (1..days_count + 1).rev() {
            numeric_days.push(i);
        }

        let mut days: Vec<String> = vec![];
        for day in 0..(6 * 7) {
            if day >= (start_offset  as usize) && !numeric_days.is_empty() {
                let number = numeric_days.pop().unwrap_or(0);
                let mut day_string = number.to_string();
                if number == today {
                    day_string = format!("[{}]", day_string);
                }
                days.push(day_string);
            }else {
                days.push("".to_string());
            }
        }

        days
    }

}


pub fn print_calendar(calendar: &mut Calendar) {
    let month_start_point = ptime::from_persian_date(
        calendar.this_year,
        calendar.this_month,
        1
    ).unwrap();

    let days_count = constants::get_day_count(
        calendar.this_month,
        month_start_point.is_leap()
    );

    calendar.days = Calendar::days(month_start_point.tm_wday, days_count, calendar.today);

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
        constants::get_season_name(calendar.this_month)
    ).bold().to_string();

    let table = builder.build()
        .with(Header(formatted_header))
        .with(Style::extended())
        .with(Segment::all().modify().with(Alignment::center()));
    println!("\n{}", table);
    println!("  Shams by Benyamin Eskandari - Benyaamin.com\n");
}
