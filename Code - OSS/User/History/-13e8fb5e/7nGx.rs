pub enum Sex {
    Male,
    Female
}

pub enum Places{
    Work,
    Park,
    Home,
    Restaurant,
}

pub enum JobType {
    LowTier,
    MidTier,
    HighTier,
}

pub struct JobHandler {
    pub id: u32,
    pub income: u32,
    pub job_type: JobType,
    pub max_positions: u32,
    pub current_workers: u32,
}

#[derive()]
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
    pub job_handler: [JobHandler; 3],
    pub days_per_year: u32,
    pub current_day: u32,
    pub days_passed_this_year: u32,
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
    pub job_id: Option<u32>,
    pub place: Places,
    pub money: u32,
    pub id: u32,
    pub age: u8,
    pub sex: Sex,
    pub partner: Option<u32>,
    pub relationships: Vec<Relationship>,
    pub traits: Traits,
    pub needs: Needs
}