use crate::people::structs::{Traits, Population};
use rand::random_range;

impl Traits {
    pub fn calc_traits(population: &Population, father: u32, mother: u32) -> Traits {
        let father_traits = &population.find_by_id(father).traits;
        let mother_traits = &population.find_by_id(mother).traits;

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