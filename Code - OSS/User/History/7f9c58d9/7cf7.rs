use crate::people::structs::{Traits};
use rand::random_range;

impl Traits {
    pub fn calc_traits(father: u32, mother: u32) -> Traits {
        
    }

    fn inherit(father: f32, mother: f32) -> f32 {
        (father + mother + random_range(-1.0..=1.0)) / 3.0
    }
}