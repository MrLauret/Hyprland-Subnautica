use crate::terminal::terminal::{terminal_setup, enter_to_continue};
use std::io::stdin;

pub struct StarterOptions {
    pub starting_humans: u32,
    pub days_per_year: u32
}

fn input(prompt: &str) -> u32 {
    let mut raw_string = String::new();

    loop {
        //Setup\\
        terminal_setup();
        raw_string.clear();

        //Getting input\\
        println!("{prompt}");
        stdin().read_line(&mut raw_string).expect("Failed to read line");

        //Sanitize the string\\
        let curated_str = raw_string.trim();

        match curated_str.parse::<u32>() {
            Err(_) => {
                terminal_setup();
                println!("Input must be a number");
                enter_to_continue();
                continue;
            }
            Ok(u) => {
                return u
            }
        };
    };
}

pub fn user_input() -> StarterOptions {
    let starter_options = StarterOptions {
        starting_humans: input("Starting humans (10 recommended): "),
        days_per_year: input("Days per year (recommended 7): ")
    };

    terminal_setup();

    println!("Starting humans: {}", &starter_options.starting_humans);

    starter_options
}