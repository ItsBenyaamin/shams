extern crate ptime;
mod shams;
use shams::calendar;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

fn main() -> Result {
    let persian_calendar = ptime::now();
    let mut calendar = calendar::Calendar::new(&persian_calendar);
    calendar::print_calendar(&mut calendar);
    Ok(())
}