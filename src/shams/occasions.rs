#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Occasions {
    day: u8,
    events: Vec<Occasion>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Occasion {
    title: String,
    is_holiday: bool
}