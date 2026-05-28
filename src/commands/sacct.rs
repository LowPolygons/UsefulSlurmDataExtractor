use std::{
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::{DateTime, Duration, Utc};

use crate::{
    cli::FilterOptions,
    commands::command::CommandCall,
    containers::{
        sacct_data::{SacctData, SacctJob, SacctStep, SacctTresAllocReq},
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
    fn command(&self, _: &SlurmData) -> Result<(), ()> {
        let start_time: String;

        let target_data = if let Some(days) = self.days {
            Utc::now() - Duration::days(days as i64)
        } else {
            Utc::now() - Duration::days(100)
        };

        start_time = target_data.format("%Y-%m-%d").to_string();

        let sacct_output = Command::new("sacct")
            .args(["--user", &self.user])
            .args(["--starttime", &start_time])
            .arg("--json")
            .output();
        let json_result: String;

        match sacct_output {
            Ok(v) => {
                json_result = String::from_utf8_lossy(&v.stdout).to_string();
            }
            Err(_) => {
                println!("Failed to run sacct command");
                return Err(());
            }
        }

        let structure: SacctData = json_string_to_struct(json_result).map_err(|_e| {
            println!("Error creating sacct struct");
            return ();
        })?;

        let filtered_jobs = match &self.filter {
            Some(filter_choice) => {
                if let Some(filter_obj) = get_filter_object(&filter_choice, self.values.clone()) {
                    structure
                        .jobs
                        .clone()
                        .into_iter()
                        .filter(|job| filter_obj.does_job_meet_filter_reqs(job))
                        .collect()
                } else {
                    structure.jobs.clone()
                }
            }
            None => structure.jobs.clone(),
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
