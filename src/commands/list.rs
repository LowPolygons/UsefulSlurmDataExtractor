use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::DateTime;

use crate::{containers::slurm_data::SlurmData, utils::secs_to_nice_time::secs_to_nice_time};

pub fn command(slurm_data: &SlurmData) -> Result<(), ()> {
    slurm_data
        .jobs
        .iter()
        .try_for_each(|job_data| -> Result<(), ()> {
            println!("==========================");
            println!("Job Name & ID: {}, {}", job_data.name, job_data.job_id);
            println!(
                "User Name and ID: {}, {}",
                job_data.user_name, job_data.user_id
            );
            println!("--------------------------");
            println!(
                "Submit Time: {}",
                DateTime::from_timestamp(job_data.submit_time as i64, 0)
                    .expect("Could not determine")
            );
            println!(
                "Latest Start Time: {}",
                DateTime::from_timestamp(job_data.start_time as i64, 0)
                    .expect("Could not determine")
            );
            println!("Job status: {}", job_data.job_state);
            if job_data.job_state == "RUNNING" {
                println!(
                    "Running Time: {}",
                    secs_to_nice_time(
                        SystemTime::now()
                            .duration_since(
                                UNIX_EPOCH + Duration::from_secs(job_data.start_time as u64)
                            )
                            .map_err(|_| ())?
                    )
                );
            }
            Ok(())
        })
        .map_err(|_| ())?;

    println!("==========================");
    println!("Listed info for {} jobs", slurm_data.jobs.len());
    println!("==========================");

    Ok(())
}
