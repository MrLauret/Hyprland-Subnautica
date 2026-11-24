use crate::terminal::terminal_setup;
use std::io::stdin;

struct StarterOptions {
    total_days: u32
}

pub fn user_input() -> StarterOptions {
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
            Ok(d) => {
                return StarterOptions { total_days: d }
            }
        };

    };
}