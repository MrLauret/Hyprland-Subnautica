pub struct Needs {
    pub money_need: u8,
    pub food_need: u8,
    pub rest_need: u8,
    pub social_need: u8
}

pub struct Opinions {
    pub political_view: f32,
}

pub struct Traits {
    pub social: f32,
    pub smart: f32,
    pub fit: f32
}

pub struct Relationship {
    pub person: u32,
    pub level: u32
}

pub struct Population<'a> {
    pub population: Vec<Person<'a>>,
    pub average_income: u32,
    pub total_people: u32
}

pub struct Couple {
    pub father: u32,
    pub mother: u32,
    pub combined_money: u32,
    pub can_reproduce: bool
}

pub struct Person<'a> {
    pub happiness: u8,
    pub income: u32,
    pub money: u32,
    pub id: u32,
    pub age: u8,
    pub sex: String,
    pub partner: u32,
    pub relationships: Vec<&'a Relationship>,
    pub traits: Traits,
    pub needs: Needs
}