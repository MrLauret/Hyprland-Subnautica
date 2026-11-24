use crate::people::structs::*;
use crate::terminal::input::StarterOptions;
use rand::random_range;

impl Population {
    pub fn from(starter_options: StarterOptions) -> Population {
        let mut population = Population {
            population: Vec::new(),
            couples: Vec::new(),
            job_handler: JobHandler::new(),
            days_per_year: starter_options.days_per_year, 
            current_day: 0,
            days_passed_this_year: 0
        };

        for _ in 0..starter_options.starting_humans {
            population.add(Person::random_agent(&population));
        }

        population
    }

    pub fn add(&mut self, person: Person) {
        println!("Agent {} has born", &person.id);
        self.population.push(person);
    }

    pub fn find_by_id(&self, id: u32) -> Option<&Person> {
        self.population
            .iter()
            .find(|person| person.id == id)
    }

    pub fn find_by_job_id(&self, id: u32) -> &JobHandler {
        self.job_handler
            .iter()
            .find(|job| job.id == id)
            .expect("They will find it")
    }
    
    pub fn new_day(&mut self) {
        let mut total_income: u32 = 0;
        let mut total_workers: u32 = 0;

        for person in &self.population {
            if let Some(job_id) = person.job_id {
                total_income += self.find_by_job_id(job_id).income;
                total_workers += 1
            }
        }

        for person in &mut self.population {
            let needs = &mut person.needs;
            needs.rest_need = needs.rest_need.saturating_sub(20).clamp(0, 100);
            needs.food_need = needs.food_need.saturating_add(30).min(100).clamp(0, 100);
        }

        self.current_day += 1;
        self.days_passed_this_year += 1;

        if self.days_passed_this_year >= self.days_per_year {
            self.days_passed_this_year = 0;
        }

        let average_income = if total_workers > 0 {
            total_income / total_workers
        } else {
            0
        };

        let days_left = self.days_per_year.saturating_sub(self.days_passed_this_year);

        println!("Average income: {}", average_income);
        println!("Day: {}", self.current_day);
        println!("Days left for new year's: {}", days_left)
    }

    pub fn calc_needs(&mut self) {
        for person in &mut self.population {
            let needs = &mut person.needs;

            let raw_money_need = if person.money > 600 {
                (random_range(0.0..=10.0) +
                (self.days_passed_this_year as f32 / self.days_per_year as f32)
                + (needs.food_need as f32 * 0.25)
                - (person.money as f32 * 0.015)
                ).max(0.0).round() as u8
                + if person.job_id == None {20} else { 0 }
            } else {
                ((self.days_passed_this_year as f32 / self.days_per_year as f32) * 50.0
                + 50.0
                + (needs.food_need as f32 * 0.25)
                ).max(0.0).round() as u8
                + if person.job_id == None {20} else { 0 }
            };

            let raw_social_need = (person.traits.social * 10.0).max(0.0).round() as u8;

            needs.social_need = raw_social_need.clamp(0, 100);
            needs.money_need = raw_money_need.clamp(0, 100);
        }
    }

    pub fn move_place(&mut self) {
        for person in &self.population {
            let needs = &person.needs;
            let traits = &person.traits;

            let park_score = (random_range(0.0..20.0)
                + (needs.social_need as f32 / 2.0)
                + (traits.social * 30.0)
                - (needs.rest_need as f32 / 10.0)
                - (needs.money_need as f32 / 20.0)
                ) as u32;
            
            let restaurant_score = (random_range(0.0..20.0)
                + (needs.social_need as f32 / 4.0)
                + (traits.social * 10.0)
                - (needs.rest_need as f32 / 10.0)
                - (needs.money_need as f32 / 20.0)
                ) as u32;
        }
    }

    pub fn give_job(&mut self, person_id: u32) -> bool {
        let (money_need, social_trait, smart_trait, fit_trait) = match self.find_by_id(person_id) {
            Some(person) => (
                person.needs.money_need as f32,
                person.traits.social,
                person.traits.smart,
                person.traits.fit,
            ),
            None => return false,
        };

        let high_job_score: u8 = (random_range(0.0..20.0)
            + (money_need as f32 / 4.0)
            + (social_trait * 15.0)
            + (smart_trait * 50.0)).round().clamp(0.0, 100.0) as u8;
        
        let mid_job_score: u8 = (random_range(0.0..20.0)
            + (money_need as f32 / 4.0) 
            + (social_trait * 35.0)
            + (smart_trait * 30.0)).round().clamp(0.0, 100.0) as u8;
        
        let low_job_score: u8 = (random_range(0.0..20.0)
            + (money_need as f32 / 3.0)
            + (social_trait * 35.0)
            + (fit_trait * 30.0)
            + (smart_trait * 7.5)).round().clamp(0.0, 100.0) as u8;
        

        let target_job_id = if high_job_score >= random_range(0..100)
                && self.job_handler.iter().find(|j| j.id == 2).map_or(false, |j| j.is_availeable())
            {
                Some(2)
            } else if mid_job_score >= random_range(0..100)
                && self.job_handler.iter().find(|j| j.id == 1).map_or(false, |j| j.is_availeable())
            {
                Some(1)
            } else if low_job_score >= random_range(0..100)
                && self.job_handler.iter().find(|j| j.id == 0).map_or(false, |j| j.is_availeable())
            {
                Some(0)
            } else {
                None
            };


            if let Some(job_id) = target_job_id {
                if let Some(job) = self.job_handler.iter_mut().find(|j| j.id == job_id) {
                    job.current_workers += 1;
                }

                if let Some(person) = self.population.iter_mut().find(|p| p.id == person_id) {
                    person.job_id = Some(job_id);
                    return true;
                }
            }

        false

    }
}