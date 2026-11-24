use crate::people::structs::{Traits, Population};
use rand::random_range;

impl Traits {
    pub fn calc_traits(population: &Population, father: u32, mother: u32) -> Traits {
        let father = 
    }

    fn inherit(father: f32, mother: f32) -> f32 {
        (father + mother + random_range(-1.0..=1.0)) / 3.0
    }
}