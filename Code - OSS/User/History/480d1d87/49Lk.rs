use crate::people::structs::{*};
use strum::IntoEnumIterator;

impl JobHandler {
    pub fn new() -> [JobHandler; 3] {
        let mut job_array: [JobHandler; 3];

        for job_type in JobType::iter(){
            match job_type {
                JobType::LowTier => {
                    job_array[0] = JobHandler {
                        id: 0,
                        income: 700,
                        job_type: JobType::LowTier,
                        open_positions: 30,
                    }
                }
                JobType::MidTier => {
                    job_array[0] = JobHandler {
                        id: 0,
                        income: 850,
                        job_type: JobType::MidTier,
                        open_positions: 20,
                    }
                }
            }
        }

        job_array
    }
}