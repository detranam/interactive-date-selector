use chrono::{TimeDelta, prelude::*};
use clap::Parser;
use console::{Key, Term};
use std::{io, process::Command};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short, action)]
    interactive_mode: bool,

    /// Take the amount of pages required to read
    #[clap(long, short, default_value_t = 0)]
    pages: u32,

    /// Take the amount of days required to complete the reading
    #[clap(long, short, default_value_t = 0)]
    days: u32,
}

fn clear_screen() {
    if Command::new("cls").status().is_err() {
        Command::new("clear").status().unwrap();
    }
}

fn print_days(day_array: &[[u32; 7]], selected_day: u32) {
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
                print!("{}{}", empty_print, printchar);
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
                    print!("\x1b[7m{}\x1b[0m{}{}", *day, modified_spaces, printchar);
                } else {
                    print!("{}{}{}", *day, modified_spaces, printchar);
                }
            }
            day_index_in_week += 1;
        }
        day_index_in_week = 0;
        printchar = "|";
        println!()
    }
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
                print_days(&day_array, highlight_day.day());
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
            print_days(&day_array, highlight_day.day());
            return;
        }
        first_day_of_month += one_day;
    }
}

fn interactive_date(begindate: Option<DateTime<Utc>>, preamble: Option<&str>) -> DateTime<Utc> {
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

fn print_days_per_page(pages: u32, days: u32) {
    let pages_per_day = pages as f32 / days as f32;
    print!("You need to read {} pages per day", (pages_per_day.floor()));
    if pages_per_day.floor() != pages_per_day {
        print!(", and {} on the last day", (pages_per_day.ceil() as u32));
    }
    println!();
}

fn create_date_str(date: DateTime<Utc>) -> String {
    format!("{}/{}/{}", date.month(), date.day(), date.year())
}

fn main() {
    let args = Args::parse();
    if args.interactive_mode {
        println!("Interactive mode not enabled yet!");
        let starting_date = interactive_date(None, None);
        let starting_date_str =
            "Starting date: ".to_owned() + &create_date_str(starting_date).to_string();
        let ending_date = interactive_date(Some(starting_date), Some(starting_date_str.as_str()));
        let ending_date_str =
            "Ending date: ".to_owned() + &create_date_str(ending_date).to_string();
        clear_screen();
        println!("{}", starting_date_str);
        println!("{}", ending_date_str);

        println!("\nEnter pages required to be read: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to get page count");
        let page_count = input.trim().parse().expect("Failed to parse input as int");
        print_days_per_page(
            page_count,
            (ending_date - starting_date)
                .num_days()
                .try_into()
                .expect("Failed to parse ending and starting day diff"),
        );

        // clear_screen();
        // print_month();
    } else {
        if args.pages == 0 {
            eprintln!("Error: Pages must be greater than 0");
            return;
        }
        if args.days == 0 {
            eprintln!("Error: Days must be greater than 0");
            return;
        }
        let pages_per_day = args.pages as f32 / args.days as f32;
        print!("You need to read {} pages per day", (pages_per_day.floor()));
        if pages_per_day.floor() != pages_per_day {
            print!(", and {} on the last day", (pages_per_day.ceil() as u32));
        }
        println!();
    }
    // let term = Term::stdout();
    // println!("Type a character:");

    // match term.read_key() {
    //     Ok(c) => println!("You pressed: {}", c),
    //     Err(e) => eprintln!("Error: {}", e),
    // }
}
