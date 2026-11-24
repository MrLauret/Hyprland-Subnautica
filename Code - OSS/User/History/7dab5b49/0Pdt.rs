use crate::terminal::terminal::{terminal_setup, enter_to_continue};
use std::io::stdin;

pub struct StarterOptions {
    pub starting_humans: u32,
    pub days_per_year: u32
}

impl StarterOptions {
    fn new() -> Self {
        StarterOptions {
            starting_humans: 10,
            days_per_year: 14
        }
    }
}

fn get_curated_input(promt: &str) {
    let mut raw_string = String::new();
}

pub fn user_input() -> StarterOptions {
    
    let mut starter_options = StarterOptions::new();

    loop {
        //Setup\\
        terminal_setup();
        raw_string.clear();

        //Getting input\\
        println!("Starting humans (recomended=10): ");
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
            Ok(h) => {
                starter_options.starting_humans = h;
                break;
            }
        };
    };

    loop {
        //Setup\\
        terminal_setup();
        raw_string.clear();

        //Getting input\\
        println!("Days per year (recomended=14): ");
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
            Ok(d) => {
                starter_options.days_per_year = d;
                break;
            }
        };
    };

    starter_options
}