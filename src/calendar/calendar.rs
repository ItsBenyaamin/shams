use crate::calendar::column::Column;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Calendar {
    pub shamsi: String,
    pub georgian: String,
    pub columns: Vec<Column>,
}
