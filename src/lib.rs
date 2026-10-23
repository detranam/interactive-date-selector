//! # Date Selector
//!
//! This crate provides a cross-platform, CLI-centric, interactive way for a user to select a date.

use chrono::{TimeDelta, prelude::*};
use console::{Key, Term};
use std::{
    io::{self, Write},
    process::Command,
};

static DAY_OF_WEEK_STRING: &str = " Sun | Mon | Tue | Wed | Thu | Fri | Sat\n";

/// Clear the console screen
fn clear_screen() {
    if Command::new("cls").status().is_err() {
        Command::new("clear").status().unwrap();
    }
}

fn print_days(day_array: &[[u32; 7]], selected_day: u32) -> String {
    let mut print_string = String::with_capacity(DAY_OF_WEEK_STRING.len() * 5);
    let mut empty_print: &str; // = "     ";
    let mut day_index_in_week = 0;
    let mut printchar = "|";
    for week in day_array {
        for day in week {
            // modify printchar
            if day_index_in_week == 6 {
                printchar = "";
            }

            // print if the day is empty
            if *day == 0 {
                if day_index_in_week > 0 {
                    empty_print = "     ";
                } else {
                    empty_print = "     "
                }
                print_string.push_str(format!("{}{}", empty_print, printchar).as_str());
            } else {
                let digit_count = day.to_string().len();
                let modified_spaces = match digit_count {
                    1 => "   ",
                    2 => "  ",
                    _ => {
                        panic!("Digit count ??");
                    }
                };
                if *day == selected_day {
                    // Highlight the selected day
                    print_string.push_str(
                        format!(" \x1b[7m{}\x1b[0m{}{}", *day, modified_spaces, printchar).as_str(),
                    );
                } else {
                    print_string
                        .push_str(format!(" {}{}{}", *day, modified_spaces, printchar).as_str());
                }
            }
            day_index_in_week += 1;
        }
        day_index_in_week = 0;
        printchar = "|";
        print_string.push('\n');
    }
    print_string
}

fn print_month_name_centered(current_month: Month) -> String {
    let total_characters_available = DAY_OF_WEEK_STRING.len();
    let month_str = format!(" {:?} ", current_month);
    let total_free_whitespace = (total_characters_available - month_str.len()) as f32 / 2.0;
    format!(
        "{}{}{}\n",
        "=".repeat(total_free_whitespace.floor() as usize),
        month_str,
        "=".repeat(total_free_whitespace.ceil() as usize)
    )
}

fn print_month(highlight_day: DateTime<Utc>, error_message: Option<String>) -> String {
    let error_string_to_print = error_message.unwrap_or_default();
    let mut month_string = String::with_capacity(DAY_OF_WEEK_STRING.len() * 6);
    let thismonth =
        Month::try_from(u8::try_from(highlight_day.month()).unwrap()).unwrap_or(Month::January);
    month_string.push_str(print_month_name_centered(thismonth).as_str());
    month_string.push_str(DAY_OF_WEEK_STRING);
    let mut day_array = [[0; 7]; 5];
    let mut starting_week: usize = 0;
    let one_day: TimeDelta = TimeDelta::days(1);
    // Populate days in the month
    let mut first_day_of_month = Utc
        .with_ymd_and_hms(highlight_day.year(), highlight_day.month(), 1, 1, 1, 1)
        .unwrap();
    loop {
        day_array[starting_week][first_day_of_month.weekday().num_days_from_sunday() as usize] =
            first_day_of_month.day();
        if first_day_of_month.weekday() == Weekday::Sat {
            // go to next week
            if starting_week == 4 {
                //end of month
                month_string.push_str(print_days(&day_array, highlight_day.day()).as_str());
                month_string.push_str(error_string_to_print.as_str());
                return month_string;
            }
            starting_week += 1;
        }
        if first_day_of_month.day()
            == thismonth
                .num_days(first_day_of_month.year())
                .unwrap()
                .into()
        {
            month_string.push_str(print_days(&day_array, highlight_day.day()).as_str());
            month_string.push_str(error_string_to_print.as_str());
            return month_string;
        }
        first_day_of_month += one_day;
    }
}

/// Function to prompt the user to select a date interactively.
///
/// 'begindate' is the date to initially start the user at. If None, defaults to today.
/// `preamble` is a &str that will be printed before the month/days. Not used if 'None'.
///
/// # Examples
/// ```
/// let picked_date = interactive_date(None, None);
/// printf!("Date selected was: {}/{}/{}", picked_date.month(), picked_date.day(), picked_date.year());
/// ```
///
pub fn interactive_date(begindate: Option<DateTime<Utc>>, preamble: Option<&str>) -> DateTime<Utc> {
    let term = Term::stdout();
    let one_day: TimeDelta = TimeDelta::days(1);
    let one_week: TimeDelta = TimeDelta::weeks(1);
    let mut highlighted_date = begindate.unwrap_or(Utc::now());
    let mut error_message: Option<String> = None;
    loop {
        clear_screen();
        if let Some(preamble) = preamble {
            println!("{}", preamble)
        }
        print!("{}", print_month(highlighted_date, error_message));
        // Flush to ensure nothing is waiting in the buffer before we get key input
        let _ = io::stdout().flush();
        // Reset the error message before we get user key input
        error_message = None;
        match term.read_key() {
            Ok(k) => match k {
                Key::ArrowUp => highlighted_date -= one_week,
                Key::ArrowDown => highlighted_date += one_week,
                Key::ArrowLeft => highlighted_date -= one_day,
                Key::ArrowRight => highlighted_date += one_day,
                Key::Enter => return highlighted_date,
                Key::Escape => return Utc::now(),
                _ => {
                    error_message = Some(format!("\x1b[33mError parsing key '{:?}'\x1b[0m.\n", k));
                }
            },
            Err(e) => eprintln!("Error reading from key with read_key: {}", e),
        }
    }
}

#[test]
fn test_centered_month() {
    let centered_month_str = print_month_name_centered(Month::May);

    // Ensure the header has all desired pieces
    assert!(centered_month_str.contains("May"));
    assert!(centered_month_str.contains("="));
    assert!(centered_month_str.contains("\n"));

    // Ensure it is the correct length- note the +2 to account for the desired single '='
    // overlap on the sides of the month
    assert!(centered_month_str.trim().len() == DAY_OF_WEEK_STRING.trim().len() + 2);
}

#[test]
fn test_print_month() {
    // Pick a leap year month for fun, to ensure correct dates are printed
    let leap_year_date = Utc.with_ymd_and_hms(2020, 2, 1, 1, 1, 1).unwrap();
    let month_str = print_month(leap_year_date, None);

    // Ensure month and leap year day are printed
    assert!(month_str.contains("29"));
    assert!(month_str.contains("February"));

    // Ensure separator and a normal (non leap) date print
    assert!(month_str.contains("11"));
    assert!(month_str.contains("Wed"));
}
