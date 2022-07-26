use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Occasions {
    pub day: u8,
    pub events: Vec<Occasion>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Occasion {
    pub title: String,
    pub is_holiday: bool
}