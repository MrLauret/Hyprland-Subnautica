mod terminal;
mod people;

use people::structs::Population;
use terminal::input::{user_input};

fn main() {
    let option_values = user_input();
    let population = Population::from(option_values);

    loop {
        for population.population{

        }
        
    }
}