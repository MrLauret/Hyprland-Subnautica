impl Couple {
    pub fn reproduce(&mut self, population: &mut Population) {
        
        if self.combined_money > 500 && self.can_reproduce {
            let father = population.find_by_id(self.father);
            let mother = population.find_by_id(self.mother);

            let baby = Person::from(population, father, mother);
            population.add(baby);
            self.reproduce_cooldowm = population.days_per_year;
        }
    }
}