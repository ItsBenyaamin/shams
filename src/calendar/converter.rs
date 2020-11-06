pub mod converter {
    #![allow(dead_code)]

    pub mod shamsi {

        pub fn get_shamsi_month_name<'a>(month: &str) -> &'a str {
            match month {
                "01" => "Farvardin",
                "02" => "Ordibehesht",
                "03" => "Khordad",
                "04" => "Tir",
                "05" => "Mordad",
                "06" => "Shahrivar",
                "07" => "Mehr",
                "08" => "Aban",
                "09" => "Azar",
                "10" => "Dey",
                "11" => "Bahman",
                "12" => "Esfand",
                _ => "!",
            }
        }

        pub fn get_shamsi_season_from_month(month: &str) -> &str {
            match month {
                "01" | "02" | "03" => "Bahar   ",
                "04" | "05" | "06" => "Tabestan",
                "07" | "08" | "09" => "Paeez   ",
                "10" | "11" | "12" => "Zemestan",
                _ => "!",
            }
        }

        pub fn get_shamsi_day_by_name(day_name: &str) -> &str {
            match day_name {
                "شنبه" => "Shanbeh  ",
                "یکشنبه" => "1-Shanbeh",
                "دوشنبه" => "2-Shanbeh",
                "سه شنبه" => "3-Shanbeh",
                "چهارشنبه" => "4-Shanbeh",
                "پنج شنبه" => "5-Shanbeh",
                "جمعه" => "Jom'eh   ",
                _ => "!",
            }
        }
    }

    pub mod georgian {

        pub fn get_georgian_month_name<'a>(month: &str) -> &'a str {
            match month {
                "01" => "January",
                "02" => "February",
                "03" => "March",
                "04" => "April",
                "05" => "May",
                "06" => "June",
                "07" => "July",
                "08" => "August",
                "09" => "September",
                "10" => "October",
                "11" => "November",
                "12" => "December",
                _ => "!",
            }
        }

        pub fn get_georgian_season_from_month(month: &str) -> &str {
            match month {
                "01" | "02" | "03" => "Winter  ",
                "04" | "05" | "06" => "Sprint  ",
                "07" | "08" | "09" => "Summer  ",
                "10" | "11" | "12" => "Autumn  ",
                _ => "!",
            }
        }
    }

    //numbers
    pub fn fa_digit_to_en(value: &str) -> String {
        let mut digits = value.to_string();
        digits = digits.replace("۰", "0");
        digits = digits.replace("۱", "1");
        digits = digits.replace("۲", "2");
        digits = digits.replace("۳", "3");
        digits = digits.replace("۴", "4");
        digits = digits.replace("۵", "5");
        digits = digits.replace("۶", "6");
        digits = digits.replace("۷", "7");
        digits = digits.replace("۸", "8");
        digits = digits.replace("۹", "9");
        digits
    }

    pub fn arabic_digit_to_en(value: &str) -> String {
        let mut digits = value.to_string();
        digits = digits.replace("٠", "0");
        digits = digits.replace("١", "1");
        digits = digits.replace("٢", "2");
        digits = digits.replace("٣", "3");
        digits = digits.replace("٤", "4");
        digits = digits.replace("٥", "5");
        digits = digits.replace("٦", "6");
        digits = digits.replace("٧", "7");
        digits = digits.replace("٨", "8");
        digits = digits.replace("٩", "9");
        digits
    }

    pub fn format_with_correct_space(value: &str, dist: usize) -> String {
        let space_count = if dist > value.len() {
            dist - value.len()
        } else {
            0
        };
        format!("{}{}", value, " ".repeat(space_count))
    }

    pub fn set_double_digit(value: &str) -> String {
        format!("{:02}", value)
    }
}
