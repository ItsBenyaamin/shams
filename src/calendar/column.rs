use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Column {
    pub shamsi: String,
    pub georgian: String,
    pub hijri: String,
    pub is_holiday: bool,
    pub is_today: bool
}