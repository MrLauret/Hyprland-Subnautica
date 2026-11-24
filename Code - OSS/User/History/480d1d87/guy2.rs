use crate::people::structs::*;

impl JobHandler {
    pub fn new() -> [JobHandler; 3] {
        [
            JobHandler {
                id: 0,
                income: 700,
                job_type: JobType::LowTier,
                open_positions: 30,
            },
            JobHandler {
                id: 1,
                income: 850,
                job_type: JobType::MidTier,
                open_positions: 20,
            },
            JobHandler {
                id: 2, // fixed typo from 3
                income: 1000,
                job_type: JobType::HighTier,
                open_positions: 30,
            },
        ]
    }

    pub fn find_by_id(&self, id: u32) -> Option<&Person> {
        self.job_type
            .iter()
            .find(|person| person.id == id)
    }
}