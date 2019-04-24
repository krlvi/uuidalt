use regex::Regex;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("{}", handle_input(String::from(buffer.trim())));
    Ok(())
}

fn handle_input(i: String) -> String {
    let uuid = Regex::new(
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
    )
    .unwrap();
    let dashless = Regex::new(r"^[0-9a-fA-F]{32}$").unwrap();
    if uuid.is_match(&i) {
        format!(
            "{}{}{}{}{}",
            &i[0..8],
            &i[9..13],
            &i[14..18],
            &i[19..23],
            &i[24..36]
        )
    } else if dashless.is_match(&i) {
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
