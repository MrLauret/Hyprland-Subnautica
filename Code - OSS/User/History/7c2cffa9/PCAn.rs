use crate::people::structs::{Person, Population, Traits, Needs, Sex, Places};

impl Person {
    pub fn from(population: &Population, father: u32, mother: u32) -> Person {
        Person {
            happiness: 50,
            income: 0,
            age: 0,
            money: 0,
            id: population.population.len() as u32,
            job: None,
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
}