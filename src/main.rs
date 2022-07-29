extern crate ptime;
mod shams;
use shams::calendar;


fn main() {
    let persian_calendar = ptime::now();
    let mut calendar = calendar::Calendar::new(&persian_calendar);
    calendar::print_calendar(&mut calendar);
}