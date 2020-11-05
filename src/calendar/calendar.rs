use serde::{Serialize, Deserialize};
use crate::calendar::column::Column;

#[derive(Serialize, Deserialize, Debug)]
pub struct Calendar {
    pub shamsi: String,
    pub georgian: String,
    pub columns: Vec<Column>
}
