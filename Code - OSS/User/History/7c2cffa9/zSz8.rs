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
            decition_scores: DecitionScores::default_scores(),
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
            decition_scores: DecitionScores::default_scores(),
            job_id: None,
            place: Places::Home,
            partner: None,
            relationships: Vec::new(),
            traits: Traits::random_traits(),
            opinions: Opinions::random_opinions(),
            needs: Needs::default_needs(),
        }
    }
}

impl Opinions {
    pub fn calc_opinions(population: &Population, father: u32, mother: u32) -> Opinions {
        let father_opinions = population.find_by_id(father).unwrap().opinions;
        let mother_opinions = population.find_by_id(mother).unwrap().opinions;

        Opinions {
            political_view: Opinions::inherit(father_opinions.political_view, mother_opinions.political_view)
        }
    }

    pub fn random_opinions() -> Opinions {
        Opinions {
            political_view: random_range(-1.0..=1.0)
        }
    }

    fn inherit(father: f32, mother: f32) -> f32 {
        (father + mother + random_range(-1.0..=1.0)) / 3.0
    }
}

impl DecitionScores {
    pub fn default_scores() -> DecitionScores {
        DecitionScores {
            park_score: 0,
            restaurant_score: 0,
            work_score: 0,
            home_score: 0
        }
    }
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