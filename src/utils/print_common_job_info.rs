use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::DateTime;

use crate::{
    containers::useful_slurm_job_info::UsefulJobInfo, utils::secs_to_nice_time::secs_to_nice_time,
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
        get_time_from_timestamp_as_string(job_data.get_submit_time() as i64)
    );
    println!(
        "Start Time: {}",
        get_time_from_timestamp_as_string(job_data.get_start_time() as i64)
    );
    if job_data.get_job_state() == "RUNNING" {
        println!(
            "End Time: {}",
            get_time_from_timestamp_as_string(job_data.get_end_time() as i64)
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

fn get_time_from_timestamp_as_string(time: i64) -> String {
    if let Some(result) = DateTime::from_timestamp(time, 0) {
        result.to_string()
    } else {
        String::from("Could not calculate")
    }
}
