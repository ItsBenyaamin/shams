pub const FARVARDIN: i32 = 31;
pub const ORDIBEHESHT: i32 = 31;
pub const KHORDAD: i32 = 31;
pub const TIR: i32 = 31;
pub const MORDAD: i32 = 31;
pub const SHAHRIVAR: i32 = 31;
pub const MEHR: i32 = 30;
pub const ABAN: i32 = 30;
pub const AZAR: i32 = 30;
pub const DEY: i32 = 30;
pub const BAHMAN: i32 = 30;
pub const ESFAND: i32 = 29;

pub fn get_day_count(month_num: i32, is_leap: bool) -> i32 {
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

pub fn get_month_name(month: i32) -> String {
    return match month {
        0 => "Farvardin".to_string(),
        1 => "Ordibehesht".to_string(),
        2 => "Khordad".to_string(),
        3 => "Tir".to_string(),
        4 => "Mordad".to_string(),
        5 => "Shahrivar".to_string(),
        6 => "Mehr".to_string(),
        7 => "Aban".to_string(),
        8 => "Azar".to_string(),
        9 => "Dey".to_string(),
        10 => "Bahman".to_string(),
        11 => "Esfand".to_string(),
        _ => "".to_string()
    }
}