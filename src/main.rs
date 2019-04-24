#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        println!("{}", handle_input(String::from(line.unwrap().trim())));
    }
    Ok(())
}

fn handle_input(i: String) -> String {
    lazy_static! {
        static ref UUID: Regex = Regex::new(
            r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
        )
        .unwrap();
        static ref DASHLESS: Regex = Regex::new(r"^[0-9a-fA-F]{32}$").unwrap();
    }

    if UUID.is_match(&i) {
        format!(
            "{}{}{}{}{}",
            &i[0..8],
            &i[9..13],
            &i[14..18],
            &i[19..23],
            &i[24..36]
        )
    } else if DASHLESS.is_match(&i) {
        format!(
            "{}-{}-{}-{}-{}",
            &i[0..8],
            &i[8..12],
            &i[12..16],
            &i[16..20],
            &i[20..32]
        )
    } else {
        i
    }
}
