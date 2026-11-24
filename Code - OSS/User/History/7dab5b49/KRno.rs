use crate::terminal::terminal::{terminal_setup, enter_to_continue};
use std::io::stdin;

pub struct StarterOptions {
    pub starting_humans: u32,
    pub days_per_year: u32
}

fn get_curated_input(prompt: &str) -> u32 {
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
    StarterOptions {
        starting_humans: get_curated_input("Starting humans (10 recomended): "),
        days_per_year: get_curated_input("Days per year (recomended 7): ")
    }
}