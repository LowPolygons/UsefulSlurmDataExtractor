use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::DateTime;

use crate::{
    cli::FilterOptions, containers::slurm_data::SlurmData, systems::filter::get_filter_object,
    utils::secs_to_nice_time::secs_to_nice_time,
};

pub fn command(
    slurm_data: &SlurmData,
    filter: &Option<FilterOptions>,
    values: &Vec<String>,
) -> Result<(), ()> {
    let filtered_data = match filter {
        Some(filter_option) => {
            if let Some(filter) = get_filter_object(filter_option, values.clone()) {
                slurm_data
                    .jobs
                    .clone()
                    .into_iter()
                    .filter(|job| filter.does_job_meet_filter_reqs(job))
                    .collect()
            } else {
                slurm_data.jobs.clone()
            }
        }
        None => slurm_data.jobs.clone(),
    };

    filtered_data
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
    println!("Listed info for {} jobs", filtered_data.len());
    println!("==========================");

    Ok(())
}
