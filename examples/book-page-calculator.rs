use chrono::prelude::*;
use std::io;
use std::process::Command;

use date_selector::interactive_date;

// Clear the console
fn clear_screen() {
    if Command::new("cls").status().is_err() {
        Command::new("clear").status().unwrap();
    }
}

fn print_days_per_page(pages: u32, days: u32) {
    let pages_per_day = pages as f32 / days as f32;
    print!("You need to read {} pages per day", (pages_per_day.floor()));
    if pages_per_day.floor() != pages_per_day {
        let fractional_days = pages_per_day - pages_per_day.floor();
        print!(
            ", and {} on the last day",
            ((days as f32) * fractional_days).ceil()
        );
    }
    println!();
}

fn create_date_str(date: DateTime<Utc>) -> String {
    format!("{}/{}/{}", date.month(), date.day(), date.year())
}

fn main() {
    let starting_date = interactive_date(None, None);
    let starting_date_str =
        "Starting date: ".to_owned() + &create_date_str(starting_date).to_string();
    let ending_date = interactive_date(Some(starting_date), Some(starting_date_str.as_str()));
    let ending_date_str = "Ending date: ".to_owned() + &create_date_str(ending_date).to_string();
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
}
