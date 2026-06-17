use std::{
    process::Command,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use chrono::DateTime;

use crate::{
    cli::FilterOptions,
    commands::command::CommandCall,
    containers::{
        piped_input::{PipedInputHandler, StructOptions},
        sacct_data::{SacctData, SacctJob, SacctStep, SacctTresAllocReq},
        sacct_handler::SacctHandler,
        slurm_data::SlurmData,
        useful_slurm_job_info::UsefulJobInfo,
    },
    systems::filter::{get_filter_object, print_help_filter_info},
    utils::{
        json_string_to_struct::json_string_to_struct, print_common_job_info::print_common_job_info,
        secs_to_nice_time::secs_as_num_to_nice_time,
    },
};

pub struct Sacct {
    pub user: String,
    pub days: Option<i16>,
    pub filter: Option<FilterOptions>,
    pub values: Vec<String>,
}

impl CommandCall for Sacct {
    fn command(&self, slurm_data: &StructOptions) -> Result<(), ()> {
        let structure: &SacctData = match slurm_data {
            StructOptions::Slurm(slurm_data) => return Err(()),
            StructOptions::Sacct(sacct_data) => sacct_data,
            StructOptions::Sinfo(sinfo_data) => return Err(()),
        };

        let secs_since_epoch: u64 = if let Some(days) = self.days {
            SystemTime::now()
                .checked_sub(Duration::from_secs(days as u64 * 86400))
                .ok_or_else(|| ())
        } else {
            SystemTime::now()
                .checked_sub(Duration::from_secs(100 * 86400))
                .ok_or_else(|| ())
        }
        .map_err(|_| {
            println!("Failed to determine cutoff date internally");
            return ();
        })?
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_| {
            println!("Failed to convert a time since unix epoch internally");
            return ();
        })?
        .as_secs();

        let jobs_in_range: Vec<SacctJob> = structure
            .jobs
            .clone()
            .into_iter()
            .filter(|job| job.time.submission > secs_since_epoch)
            .collect();

        let filtered_jobs = match &self.filter {
            Some(filter_choice) => {
                if let Some(filter_obj) = get_filter_object(&filter_choice, self.values.clone()) {
                    jobs_in_range
                        .into_iter()
                        .filter(|job| filter_obj.does_job_meet_filter_reqs(job))
                        .collect()
                } else {
                    jobs_in_range
                }
            }
            None => jobs_in_range,
        };

        println!("============================");
        filtered_jobs.iter().try_for_each(|job| -> Result<(), ()> {
            print_common_job_info(job).map_err(|e| {
                println!("Error printing job info: {e}");

                return ();
            })?;

            if job.get_job_state() != "CANCELLED" {
                println!(
                    "End time: {}",
                    DateTime::from_timestamp(job.get_end_time() as i64, 0)
                        .expect("Could not determine")
                );
                println!(
                    "Time Limit: {}",
                    secs_as_num_to_nice_time((job.time.limit.number * 60.0) as f64)
                );

                println!(
                    "Actual Job Length: {}",
                    secs_as_num_to_nice_time((job.get_end_time() - job.get_start_time()) as f64)
                );

                println!("----------------------------");
                println!("Number of CPUs: {}", job.required.cpus);
                println!(
                    "Estimated CPU Memory Usage: ~{} GB",
                    (job.required.cpus as f64 * job.required.memory_per_cpu.number as f64) / 1024.0
                );

                steps_info_printer(job);
            }
            println!("============================");

            Ok(())
        })?;

        println!("============================");
        println!("Listed info for {} jobs", filtered_jobs.len());
        println!("============================");

        if filtered_jobs.len() == 0
            && structure.jobs.len() != 0
            && let Some(filter_choice) = &self.filter
        {
            print_help_filter_info(&structure.jobs, &filter_choice);
        }

        return Ok(());
    }

    fn get_piped_input_handler(&self) -> Box<dyn PipedInputHandler> {
        return Box::new(SacctHandler::new());
    }
}

fn steps_info_printer(job: &SacctJob) {
    if job.steps.len() == 0 {
        return;
    }

    println!("This job had {} step(s)", job.steps.len());

    let print_time_info = if job.steps.len() > 1 { true } else { false };

    let empty_backup_vec: Vec<SacctTresAllocReq> = Vec::new();

    println!("===== Step Info =====");
    job.steps.iter().for_each(|step| {
        println!("Name: {}", step.step.name);

        println!("/----/ Memory Info /-----/");
        let max = step.tres.requested.get("max").unwrap_or(&empty_backup_vec);
        let avg = step
            .tres
            .requested
            .get("average")
            .unwrap_or(&empty_backup_vec);

        max.iter().for_each(|val| {
            if val.key_is_type == "mem" {
                println!("Maximum RSS: {}K", val.count as f64 / 1024.0);
            }
            if val.key_is_type == "vmem" {
                println!("Maximum VM Size: {}K", val.count as f64 / 1024.0);
            }
        });

        let mut avg_count: i64 = 0;
        let mut max_count: i64 = 0;

        avg.iter().for_each(|val| {
            if val.key_is_type == "mem" {
                println!("Average RSS: {}K", val.count as f64 / 1024.0);
                avg_count = val.count;
            }
            if val.key_is_type == "vmem" {
                println!("Average VM Size: {}K", val.count as f64 / 1024.0);
                max_count = val.count;
            }
        });

        // * nprocs
        println!("Max RAM Usage: {}", max_count * job.required.cpus);
        println!("Average RAM Usage: {}", avg_count * job.required.cpus);

        if print_time_info {
            println!("/----/ Time /-----/");
            println!(
                "Length of step: {}",
                secs_as_num_to_nice_time(step.time.elapsed as f64)
            )
        }
    });
}
