mod terminal;
mod people;

use people::structs::Population;
use terminal::input::{user_input};

use crate::people::population;

fn main() {
    let option_values = user_input();
    let population_general = Population::from(option_values);

    loop {
        for {

        };
    }
}