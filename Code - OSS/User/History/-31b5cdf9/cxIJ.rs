use std::{io::{stdout, stdin}};
use crossterm::{ExecutableCommand, terminal, cursor};

struct StarterOptions {
    total_days: u32
}

fn terminal_setup() {
    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout().execute(cursor::MoveToRow(0)).unwrap();
}

fn user_input() -> StarterOptions {
    let mut raw_string = String::new();

    loop {
        //Setup\\
        terminal_setup();
        raw_string.clear();

        //Getting input\\
        println!("Total days: ");
        stdin().read_line(&mut raw_string).expect("Failed to read line");

        //Sanitize the string\\
        let curated_str = raw_string.trim();

        match curated_str.parse::<u32>() {
            Err(_) => {
                terminal_setup();
                println!("Input must be a number");
                println!("Press enter to continue");
                stdin().read_line(&mut raw_string).unwrap();
                continue;
            }
            Ok(d) => d
        };

        break;
    };

    StarterOptions { total_days: 1 }
}

fn main() {
    let option_values = user_input();
}