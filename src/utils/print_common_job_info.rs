use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::DateTime;

use crate::{
    containers::{slurm_data::SlurmJob, useful_slurm_job_info::UsefulJobInfo},
    utils::secs_to_nice_time::secs_to_nice_time,
};

pub fn print_common_job_info(job_data: &impl UsefulJobInfo) -> Result<(), String> {
    println!(
        "Job Name & ID: {}, {}",
        job_data.get_job_name(),
        job_data.get_job_id()
    );
    println!(
        "User Name and ID: {}, {}",
        job_data.get_user_name(),
        job_data.get_user_id()
    );
    println!("Job status: {}", job_data.get_job_state());
    println!("--------------------------");
    println!(
        "Submit Time: {}",
        DateTime::from_timestamp(job_data.get_submit_time() as i64, 0)
            .expect("Could not determine")
    );
    println!(
        "Latest Start Time: {}",
        DateTime::from_timestamp(job_data.get_start_time() as i64, 0).expect("Could not determine")
    );
    if job_data.get_job_state() == "RUNNING" {
        println!(
            "End Time: {}",
            DateTime::from_timestamp(job_data.get_end_time() as i64, 0)
                .expect("Could not determine")
        );
        println!(
            "Running Time: {}",
            secs_to_nice_time(
                SystemTime::now()
                    .duration_since(
                        UNIX_EPOCH + Duration::from_secs(job_data.get_start_time() as u64)
                    )
                    .map_err(|_| String::from("Could not calculate running time"))?
            )
        );
    }
    println!("Job directory: {}", job_data.get_directory());
    Ok(())
}
