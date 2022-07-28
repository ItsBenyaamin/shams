use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use colored_truecolor::Colorize;
use ptime::Tm;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::{constants, storage_helper};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calendar {
    pub this_year: i32,
    pub this_month: i32,
    pub week_day: i32,
    pub today: i32,
    pub day_name: String,
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
        let mut calendar = Calendar {
            this_year: pt.tm_year,
            this_month,
            week_day: pt.tm_wday,
            today: pt.tm_mday,
            day_name: constants::get_day_name(pt.tm_wday),
            month_name: constants::get_month_name(this_month),
            days_titles: days,
            days: vec![]
        };
        calendar
    }

    pub async fn load() -> Option<Calendar> {
        let file = storage_helper::read_file(constants::CALENDAR_FILE_NAME).await;
        if let Some(content) = file {
            if !content.is_empty() {
                let calendar: Calendar = serde_json::from_str(&content).unwrap();
                return Some(calendar);
            }
        }

        None
    }

    pub fn days(start_offset: i32, days_count: i32) -> Vec<String> {
        let mut numeric_days = vec![];
        for i in (1..days_count + 1).rev() {
            numeric_days.push(i);
        }

        let mut days: Vec<String> = vec![];
        for day in 0..(6 * 7) {
            if day >= (start_offset  as usize) && !numeric_days.is_empty() {
                let pd = numeric_days.pop().unwrap_or(0);
                days.push(pd.to_string());
            }else {
                days.push("".to_string());
            }
        }

        days
    }

}

