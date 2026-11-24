use crate::people::structs::*;

impl JobHandler {
    pub fn new() -> [JobHandler; 3] {
        [
            JobHandler {
                id: 0,
                income: 150,
                job_type: JobType::LowTier,
                max_positions: 20,
                current_workers: 0
            },
            JobHandler {
                id: 1,
                income: 200,
                job_type: JobType::MidTier,
                max_positions: 20,
                current_workers: 0
            },
            JobHandler {
                id: 2,
                income: 350,
                job_type: JobType::HighTier,
                max_positions: 10,
                current_workers: 0
            },
        ]
    }

    pub fn is_availeable(&self) -> bool {
        if self.current_workers == self.max_positions 
    }
}