use std::process::Command;

use crate::{
    commands::command::CommandCall,
    containers::{sacct_data::SacctData, slurm_data::SlurmData},
    utils::json_string_to_struct::json_string_to_struct,
};

pub struct Sacct {
    pub username: String,
    pub year: Option<i16>,
    pub month: Option<i8>,
    pub day: Option<i8>,
}

impl CommandCall for Sacct {
    fn command(&self, _: &SlurmData) -> Result<(), ()> {
        let start_time: String;

        if let Some(year) = self.year
            && let Some(month) = self.month
            && let Some(day) = self.day
        {
            start_time = format!("{}-{}-{}", year, month, day);
        } else if self.year.is_none() && self.month.is_none() && self.day.is_none() {
            start_time = String::from("2026-01-01");
        } else {
            println!(
                "When using this command, you should either provide all 3 optional time arguments, or none"
            );
            return Err(());
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

        structure.jobs.iter().for_each(|job| {
            println!("=========================");
            println!("User: {}", job.user);
            println!("Directory: {}", job.working_directory);
            println!("Return Status: {}", job.exit_code.status);
        });

        return Ok(());
    }
}
