use crate::people::structs::{Needs};

impl Needs {
    fn default_needs() -> Needs {
        Needs {
            money_need: 0,
            food_need: 0,
            rest_need: 0,
            social_need: 0
        }
    }
}