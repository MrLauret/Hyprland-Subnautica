mod terminal;
mod people;

use people::structs::Population;
use terminal::input::{user_input};

fn main() {
    let option_values = user_input();
    let population_global = Population::from(option_values);

    loop {
        for Population { population, couples, days_per_year, day, average_income }
        
    }
}