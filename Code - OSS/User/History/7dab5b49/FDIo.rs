use crate::terminal::terminal_setup;
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

pub fn user_input() -> StarterOptions {
    let mut raw_string = String::new();
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
                println!("Press enter to continue");
                stdin().read_line(&mut raw_string).unwrap();
                continue;
            }
            Ok(h) => {
                starter_options.starting_humans = h;
            }
        };

        return starter_options
    };
}