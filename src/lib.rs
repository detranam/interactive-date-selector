use chrono::{TimeDelta, prelude::*};
use console::{Key, Term};
use std::process::Command;

pub fn clear_screen() {
    if Command::new("cls").status().is_err() {
        Command::new("clear").status().unwrap();
    }
}

fn print_days(day_array: &[[u32; 7]], selected_day: u32) -> String {
    let mut print_string = String::with_capacity(40*5);
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
                print_string.push_str(format!("{}{}",empty_print,printchar).as_str());
                // print!("{}{}", empty_print, printchar);
            } else {
                let digit_count = day.to_string().len();
                let modified_spaces = match digit_count {
                    1 => "    ",
                    2 => "   ",
                    _ => {
                        panic!("Digit count ??");
                    }
                };
                if *day == selected_day {
                    print_string.push_str(format!("\x1b[7m{}\x1b[0m{}{}", *day, modified_spaces, printchar).as_str());
                    // print!("\x1b[7m{}\x1b[0m{}{}", *day, modified_spaces, printchar);
                } else {
                    print_string.push_str(format!("{}{}{}", *day, modified_spaces, printchar).as_str());
                    // print!("{}{}{}", *day, modified_spaces, printchar);
                }
            }
            day_index_in_week += 1;
        }
        day_index_in_week = 0;
        printchar = "|";
        print_string.push('\n');
        // println!()
    }
    print_string
}

fn print_month(highlight_day: DateTime<Utc>) {
    // let highlight_day = Utc::now();
    let thismonth =
        Month::try_from(u8::try_from(highlight_day.month()).unwrap()).unwrap_or(Month::January);
    println!("{:?}", thismonth);
    let days_string = " Sun | Mon | Tue | Wed | Thu | Fri | Sat";
    println!("{}", days_string);
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
                print!("{}",print_days(&day_array, highlight_day.day()));
                return;
            }
            starting_week += 1;
        }
        // make 'today' into 'tomorrow'
        if first_day_of_month.day()
            == thismonth
                .num_days(first_day_of_month.year())
                .unwrap()
                .into()
        {
            print!("{}",print_days(&day_array, highlight_day.day()));
            return;
        }
        first_day_of_month += one_day;
    }
}

pub fn interactive_date(begindate: Option<DateTime<Utc>>, preamble: Option<&str>) -> DateTime<Utc> {
    let term = Term::stdout();
    let one_day: TimeDelta = TimeDelta::days(1);
    let one_week: TimeDelta = TimeDelta::weeks(1);
    let mut highlighted_date = begindate.unwrap_or(Utc::now());
    loop {
        clear_screen();
        if let Some(preamble) = preamble {
            println!("{}", preamble)
        }

        print_month(highlighted_date);
        match term.read_key() {
            Ok(k) => match k {
                Key::ArrowUp => highlighted_date -= one_week,
                Key::ArrowDown => highlighted_date += one_week,
                Key::ArrowLeft => highlighted_date -= one_day,
                Key::ArrowRight => highlighted_date += one_day,
                Key::Enter => return highlighted_date,
                Key::Escape => return Utc::now(),
                _ => panic!("Ahh bad key!"),
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
