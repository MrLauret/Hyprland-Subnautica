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

pub struct Population {
    pub population: Vec<Person>,
    pub couples: Vec<Couple>,
    pub days_per_year: u32,
    pub day: u32,
    pub average_income: u32,
}

pub struct Couple {
    pub father: u32,
    pub mother: u32,
    pub combined_money: u32,
    pub reproduce_cooldowm: u32,
    pub can_reproduce: bool
}

pub struct Person {
    pub happiness: u8,
    pub income: u32,
    pub money: u32,
    pub id: u32,
    pub age: u8,
    pub sex: String,
    pub job: Option<String>,
    pub partner: Option<u32>,
    pub relationships: Vec<Relationship>,
    pub traits: Traits,
    pub needs: Needs
}