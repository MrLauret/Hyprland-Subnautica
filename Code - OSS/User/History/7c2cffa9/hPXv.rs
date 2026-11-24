use crate::people::structs::{*};
use rand::random_range;

impl Person {
    pub fn from(population: &Population, father: u32, mother: u32) -> Person {
        Person {
            happiness: 50,
            age: 0,
            money: 0,
            id: population.population.len() as u32,
            job_id: None,
            place: Places::Home,
            sex: Person::random_sex(),
            partner: None,
            relationships: Vec::new(),
            traits: Traits::calc_traits(population, father, mother),
            needs: Needs::default_needs()
        }
    }

    fn random_sex() -> Sex {
        let sex: Sex;

        if rand::random_range(0..=1) == 1 {
            sex = Sex::Male;
        } else {
            sex = Sex::Female;
        }

        sex
    }

    pub fn random_agent(population: &Population) -> Person {
        Person {
            happiness: 50,
            money: 500,
            id: population.population.len() as u32,
            age: 18,
            sex: Person::random_sex(),
            job_id: None,
            place: Places::Home,
            partner: None,
            relationships: Vec::new(),
            traits: Traits::random_traits(),
            needs: Needs::default_needs(),
        }
    }

    fn calc_needs(&self) {
        self.needs.money_need = if self.money > 600 {
            (random_range(0.00..=20.00) + (self.needs.food_need as f32 * 0.25)) as u8
        } else {
            
        }
    }
}

impl Opinions {
    
}

impl Traits {
    pub fn calc_traits(population: &Population, father: u32, mother: u32) -> Traits {
        let father_traits = &population.find_by_id(father).unwrap().traits;
        let mother_traits = &population.find_by_id(mother).unwrap().traits;

        Traits {
            social: Traits::inherit(father_traits.social, mother_traits.social),
            smart: Traits::inherit(father_traits.smart, mother_traits.smart),
            fit: Traits::inherit(father_traits.fit, mother_traits.fit),
        }
    }

    pub fn random_traits() -> Traits {
        Traits {
            social: random_range(-1.0..=1.0),
            smart: random_range(-1.0..=1.0),
            fit: random_range(-1.0..=1.0),
        }
    }

    fn inherit(father: f32, mother: f32) -> f32 {
        (father + mother + random_range(-1.0..=1.0)) / 3.0
    }

}

impl Needs {
    pub fn default_needs() -> Needs {
        Needs {
            money_need: 0,
            food_need: 0,
            rest_need: 0,
            social_need: 0
        }
    }
}

impl Couple {
    pub fn reproduce(&mut self, population: &mut Population) {
        
        if self.combined_money > 500 && self.can_reproduce {

            let baby = Person::from(population, self.father, self.mother);
            population.add(baby);
            self.reproduce_cooldowm = population.days_per_year;
        }
    }
}