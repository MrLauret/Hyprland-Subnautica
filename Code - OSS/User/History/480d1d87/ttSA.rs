use crate::people::structs::*;

impl JobHandler {
    pub fn new() -> [JobHandler; 3] {
        [
            JobHandler {
                id: 0,
                income: 150,
                job_type: JobType::LowTier,
                open_positions: 20,
            },
            JobHandler {
                id: 1,
                income: ,
                job_type: JobType::MidTier,
                open_positions: 20,
            },
            JobHandler {
                id: 2,
                income: 350,
                job_type: JobType::HighTier,
                open_positions: 10,
            },
        ]
    }
}