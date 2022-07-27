
pub const OCCASIONS_URL: &str = "https://projects.benyaamin.com/projects/shams";
pub const SHAMS_FOLDER_NAME: &str = "shams";
pub const OCCASIONS_FILE_NAME: &str = "occasions.json";
pub const CALENDAR_FILE_NAME: &str = "cache.json";
pub const CONFIG_FILE_NAME: &str = "config.json";


pub const FARVARDIN: u8 = 31;
pub const ORDIBEHESHT: u8 = 31;
pub const KHORDAD: u8 = 31;
pub const TIR: u8 = 31;
pub const MORDAD: u8 = 31;
pub const SHAHRIVAR: u8 = 31;
pub const MEHR: u8 = 30;
pub const ABAN: u8 = 30;
pub const AZAR: u8 = 30;
pub const DEY: u8 = 30;
pub const BAHMAN: u8 = 30;
pub const ESFAND: u8 = 29;

pub fn get_day_count(month_num: u8, is_leap: bool) -> u8 {
    return match month_num {
        0 => FARVARDIN,
        1 => ORDIBEHESHT,
        2 => KHORDAD,
        3 => TIR,
        4 => MORDAD,
        5 => SHAHRIVAR,
        6 => MEHR,
        7 => ABAN,
        8 => AZAR,
        9 => DEY,
        10 => BAHMAN,
        11 => {
            if is_leap { ESFAND + 1 }
            else { ESFAND }
        },
        _ => 0
    }
}

pub const SHANBE: &str = "Shanbe";
pub const YEK: &str = "Yek";
pub const DO: &str = "Do";
pub const SE: &str = "Se";
pub const CHAHAR: &str = "Chahar";
pub const PANJ: &str = "Panj";
pub const ADINEH: &str = "Adineh";

pub fn get_day_name<'a>(day_of_week: u8) -> &'a str {
    return match day_of_week {
        0 => SHANBE,
        1 => YEK,
        2 => DO,
        3 => SE,
        4 => CHAHAR,
        5 => PANJ,
        6 => ADINEH,
        _ => ""
    }
}