use chrono::DateTime;

use crate::containers::slurm_data::SlurmData;

pub fn command(slurm_data: &SlurmData) -> Result<(), ()> {
    slurm_data.jobs.iter().for_each(|job_data| {
        println!("==========================");
        println!("Job Name & ID: {}, {}", job_data.name, job_data.job_id);
        println!(
            "User Name and ID: {}, {}",
            job_data.user_name, job_data.user_id
        );
        println!("--------------------------");
        println!(
            "Submit Time: {}",
            DateTime::from_timestamp(job_data.submit_time as i64, 0).expect("Could not determine")
        );
        println!(
            "Latest Start Time: {}",
            DateTime::from_timestamp(job_data.start_time as i64, 0).expect("Could not determine")
        );
        println!("Job status: {}", job_data.job_state);
    });

    println!("==========================");
    println!("Listed info for {} jobs", slurm_data.jobs.len());
    println!("==========================");

    Ok(())
}
