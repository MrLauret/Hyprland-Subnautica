mod terminal;
mod people;
mod economy;

use people::structs::Population;
use terminal::input::{user_input};

fn main() {
    let option_values = user_input();
    let mut population_general = Population::from(option_values);

    loop {
        for person in &mut population_general.population.iter(){
            population_general.calc_stats();
            population_general.new_day();
            population_general.calc_needs();
        };
    }
}