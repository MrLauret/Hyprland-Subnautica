use crate::people::structs::{*};
use strum::IntoEnumIterator;

impl JobHandler {
    pub fn new() -> [JobHandler; 3] {
        let job_array: [JobHandler; 3];

        for job_type in JobType::iter(){
            match job_type {
                JobType::LowTier => {

                }
            }
        }

        job_array
    }
}