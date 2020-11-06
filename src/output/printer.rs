pub mod printer {
    use crate::calendar::calendar::Calendar;
    use crate::calendar::column::Column;
    use crate::calendar::converter::converter::*;
    use colored_truecolor::Colorize;

    pub fn print(calendar: &Calendar) {
        print_header(&calendar.shamsi, &calendar.georgian);
        print_columns(&calendar.columns);
        print_footer();
    }

    pub fn print_header(top_shamsi: &str, top_georgian: &str) {
        println!("+--------------------------------------------------------------+");
        println!("|{}|", format_with_correct_space(&top_shamsi, 71));
        println!("|{}|", format_with_correct_space(&top_georgian, 71));
        println!("+--------------------------------------------------------------+");
        println!(
            "| {} |  {}   |   {}   |   {}   | {} |  {}  | {} |",
            "Shanbe".bold(),
            "Yek".bold(),
            "Do".bold(),
            "Se".bold(),
            "Chahar".bold(),
            "Panj".bold(),
            "Adineh".bold(),
        );
        println!("+--------------------------------------------------------------+");
    }

    pub fn print_columns(columns: &[Column]) {
        print_column(columns[0..7].to_vec());
        print_column(columns[7..14].to_vec());
        print_column(columns[14..21].to_vec());
        print_column(columns[21..28].to_vec());
        print_column(columns[28..35].to_vec());
    }

    fn print_column(columns: Vec<Column>) {
        let mut shamsi_part = String::from("");
        let mut other_parts = String::from("");
        columns.iter().for_each(|c| {
            let mut shamsi = set_double_digit(&c.shamsi);
            let georgian = set_double_digit(&c.georgian);
            let hijri = set_double_digit(&c.hijri);
            if c.is_holiday {
                shamsi = shamsi.as_str().red().to_string();
            }
            if c.is_today {
                shamsi_part.push_str(
                    format!(
                        "  {}{}{}  ",
                        "(".bold().yellow(),
                        shamsi.as_str().bold(),
                        ")".bold().yellow()
                    )
                    .as_str(),
                );
            } else {
                shamsi_part.push_str(format!("   {}   ", shamsi).as_str());
            }
            shamsi_part.push_str("|");
            other_parts.push_str(format!(" {}  {} ", hijri, georgian).as_str());
            other_parts.push_str("|");
        });
        println!("|{}", shamsi_part);
        println!("|{}", other_parts);
        println!("+--------------------------------------------------------------+");
    }

    pub fn print_footer() {
        print!(" Powered by Time.ir - ");
        print!("{}", "@GrayMind75".green());
        println!("\r\n");
    }

    pub fn print_help() {
        println!("Usage: shams [options]");
        println!("  -h, --help      show help and exit");
        println!("  -u, --update    update the cache data");
        println!("  -a, --about     some words about program");
    }

    pub fn print_about() {
        println!("this program is a Rust port of 'pouriya/tir' project on github written in python by Pouriya");
        println!("-Programmer: Benyamin Choukan");
        println!("-Email:      me@benyaamin.com");
        println!("-Website:    https://benyaamin.com");
        println!("-Github:     https://github.com/graymind75");
    }
}
