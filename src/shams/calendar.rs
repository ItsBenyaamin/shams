use serde::{Serialize, Deserialize};
use crate::shams::occasions::Occasions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calendar<'a> {
    pub this_year: u16,
    pub this_month: u8,
    pub today: u8,
    pub day_name: &'a str,
    pub month_name: &'a str,
    pub columns: Vec<Column>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub jalali_day: u8,
    pub georgian_day: u8,
    pub lunar_day: u8,
    pub is_holiday: bool
}

impl Calendar {

}