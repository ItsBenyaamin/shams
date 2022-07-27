use serde::{Serialize, Deserialize};
use crate::{constants, storage_helper};

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

impl Occasions {

    pub fn save_occasions(&self) -> Option<()> {
        if let Some(occasions) = _occasions {
            let json_string = serde_json::to_string(&occasions).unwrap();
            storage_helper::write_to_file(constants::OCCASIONS_FILE_NAME, json_string).await;
            return Some(())
        }

        None
    }

}

impl ToString for Occasions {

    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

}