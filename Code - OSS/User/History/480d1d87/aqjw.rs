use crate::people::structs::*;

impl JobHandler {
    pub fn new() -> [JobHandler; 3] {
        [
            JobHandler {
                id: 0,
                income: 150,
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
}