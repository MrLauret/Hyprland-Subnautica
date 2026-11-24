use crate::people::structs::{Couple, Population, Person};

impl Couple {
    pub fn reproduce(&mut self, population: &mut Population) {
        
        if self.combined_money > 500 && self.can_reproduce {

            let baby = Person::from(population, self.father, self.mother);
            population.add(baby);
            self.reproduce_cooldowm = population.days_per_year;
        }
    }
}