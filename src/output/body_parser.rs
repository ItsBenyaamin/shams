pub mod body_parser {
    use scraper::{Html, Selector, ElementRef};
    use crate::consts::consts;
    use crate::calendar::calendar::Calendar;
    use crate::calendar::column::Column;
    use crate::calendar::converter::converter::*;
    use crate::calendar::converter::converter::shamsi::*;
    use crate::calendar::converter::converter::georgian::*;
    use colored_truecolor::Colorize;

    pub const ERR: &str = "there is an error in getting info from time.ir, It's gonna fixed soon :).";

    pub async fn process_the_body(html: String) -> Calendar {
        let body = Html::parse_fragment(&html);
        let top_shamsi = get_today_as_shamsi(&body).await;
        let top_georgian = get_today_as_georgian(&body).await;
        let columns_list = get_columns_list(&body).await;
        let calendar = Calendar {
            shamsi: top_shamsi,
            georgian: top_georgian,
            columns: columns_list
        };
        calendar
    }

    async fn get_today_as_shamsi(body: &Html) -> String {
        let mut date = get_value_from_body(&body, consts::SHAMSI_DATE_SELECTOR).await;
        date = fa_digit_to_en(date.as_str());
        let full_date = get_value_from_body(&body, consts::SHAMSI_FULL_DATE_SELECTOR).await;
        let day_name = full_date.split("-").collect::<Vec<&str>>()[0].trim().to_string();
        let splinted_date = date.split("/").collect::<Vec<&str>>();
        let month = format!("{}({})", get_shamsi_month_name(splinted_date[1]), splinted_date[1]);

        let formatted_value = format!(
            "{}: {}  {}  {} {}  {}",
            "Emruz".green(),
            get_shamsi_day_by_name(day_name.as_str()),
            splinted_date[2],
            format_with_correct_space(&month, 14),
            splinted_date[0],
            get_shamsi_season_from_month(splinted_date[1])
        );
        formatted_value
    }

    async fn get_today_as_georgian(body: &Html) -> String {
        let mut date = get_value_from_body(&body, consts::GEORGIAN_DATE_SELECTOR).await;
        date = fa_digit_to_en(date.as_str());
        let full_date = get_value_from_body(&body, consts::GEORGIAN_FULL_DATE_SELECTOR).await;
        let day_name = full_date.split("-").collect::<Vec<&str>>()[0].trim().to_string();
        let splinted_date = date.split("-").collect::<Vec<&str>>();
        let month = format!("{}({})", get_georgian_month_name(splinted_date[1]), splinted_date[1]);
        let formatted_value = format!(
            "{}: {}  {}  {} {}  {}",
            "Today".green(),
            format_with_correct_space(&day_name, consts::DAY_SPACE_SIZE),
            splinted_date[2],
            format_with_correct_space(&month, consts::MONTH_SPACE_SIZE),
            splinted_date[0],
            get_georgian_season_from_month(splinted_date[1])
        );
        return formatted_value
    }

    async fn get_columns_list(body: &Html) -> Vec<Column> {
        let mut columns: Vec<Column> = vec![];
        let selector = Selector::parse(consts::CALENDAR_ID).expect("cant find calendar");
        let parent_element = body.select(&selector).next().unwrap();
        let divs_selector = Selector::parse("div[style=\"width:14.28%\"]").unwrap();
        let divs_in_calendar = parent_element.select(&divs_selector);
        divs_in_calendar.for_each(|x| {
            let _is_today = x.value().classes().filter(|class| { class.trim().eq("today") }).count() > 0;
            let _is_holiday = x.children().next().unwrap().value().as_element().unwrap()
                .classes().filter(|class| { class.to_string().eq("holiday") }).count() > 0;
            let shamsi_value = get_inner_html(x, "div>div.jalali");
            let georgian_value = get_inner_html(x, "div>div.miladi");
            let hijri_value = get_inner_html(x, "div>div.qamari");

            columns.push(Column {
                is_today: _is_today,
                is_holiday: _is_holiday,
                shamsi: fa_digit_to_en(&shamsi_value),
                hijri: arabic_digit_to_en(&hijri_value),
                georgian: georgian_value
            });
        });

        columns
    }

    async fn get_value_from_body(body: &Html, selector_key: &str) -> String {
        let selector = Selector::parse(selector_key).expect(ERR);
        body.select(&selector).next().unwrap().text().collect::<Vec<_>>().concat()
    }

    fn get_inner_html(body: ElementRef<'_>, selector_key: &str) -> String {
        let selector = Selector::parse(selector_key).unwrap();
        body.select(&selector).next().unwrap().inner_html()
    }

}