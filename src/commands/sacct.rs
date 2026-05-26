use std::process::Command;

use chrono::{Duration, Utc};

use crate::{
    cli::FilterOptions,
    commands::command::CommandCall,
    containers::{sacct_data::SacctData, slurm_data::SlurmData},
    systems::filter::{get_filter_object, print_help_filter_info},
    utils::{
        json_string_to_struct::json_string_to_struct, print_common_job_info::print_common_job_info,
    },
};

pub struct Sacct {
    pub username: String,
    pub backlog_days: Option<i16>,
    pub filter: Option<FilterOptions>,
    pub values: Vec<String>,
}

impl CommandCall for Sacct {
    fn command(&self, _: &SlurmData) -> Result<(), ()> {
        let start_time: String;

        if let Some(days) = self.backlog_days {
            let target_data = Utc::now() - Duration::days(days as i64);

            start_time = target_data.format("%Y-%m-%d").to_string();
        } else {
            start_time = String::from("2026-01-01");
        }

        let sacct_output = Command::new("sacct")
            .args(["--user", &self.username])
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
