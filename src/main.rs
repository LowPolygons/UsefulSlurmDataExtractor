use std::io::{self, IsTerminal, Read};

use chrono::DateTime;
use serde::de::DeserializeOwned;

mod slurm_data;

pub fn json_string_to_struct<T: DeserializeOwned>(stringy_json: String) -> Result<T, ()> {
    let structy_value = serde_json::from_str(&stringy_json).map_err(|_| {
        return ();
    })?;

    Ok(structy_value)
}

fn main() {
    let mut input = String::new();

    if io::stdin().is_terminal() {
        println!(
            "User did not provide any input - did you run it like 'squeue --json | UsefulSlurmDataExtractor' ?"
        );
        return ();
    }

    let _ = io::stdin().read_to_string(&mut input).map_err(|_| {
        println!("Failed to read user input - did you run it like 'squeue --json | UsefulSlurmDataExtractor' ?");
        return ();
    });

    let structure: slurm_data::SlurmData = match json_string_to_struct(input) {
        Ok(val) => val,
        Err(_) => {
            println!(
                "Failed to format input properly - did you run it like 'squeue --json | UsefulSlurmDataExtractor' ?"
            );
            return ();
        }
    };

    structure.jobs.iter().for_each(|job_data| {
        println!("==========================");
        println!("Job Name & ID: {}, {}", job_data.name, job_data.job_id);
        println!(
            "User Name and ID: {}, {}",
            job_data.user_name, job_data.user_id
        );
        println!("--------------------------");
        println!(
            "Submit Time: {}",
            DateTime::from_timestamp(job_data.submit_time as i64, 0).expect("Conversion Failure")
        );
        println!(
            "Latest Start Time: {}",
            DateTime::from_timestamp(job_data.start_time as i64, 0).expect("Conversion Failure")
        );
    });

    println!("==========================");
    println!("Listed info for {} jobs", structure.jobs.len());
    println!("==========================");
}
