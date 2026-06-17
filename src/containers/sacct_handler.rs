use std::process::Command;

use chrono::{Duration, Utc};

use crate::{
    containers::{piped_input::PipedInputHandler, sacct_data::SacctData},
    utils::json_string_to_struct::json_string_to_struct,
};

use super::piped_input::StructOptions;

pub struct SacctHandler {}

impl SacctHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl PipedInputHandler for SacctHandler {
    fn try_make_piped_input_into_struct(&self, input: String) -> Result<StructOptions, String> {
        let structure: SacctData = json_string_to_struct(input).map_err(|_| {
            "Piped input could not be mapped to struct - did you pipe it in correctly?".to_string()
        })?;

        Ok(StructOptions::Sacct(structure))
    }

    fn try_run_command_to_get_struct(
        &self,
        args: std::collections::HashMap<String, String>,
    ) -> Result<StructOptions, String> {
        let result_string: String;

        let user = args
            .get("user")
            .ok_or_else(|| ())
            .map_err(|_| "Could not find the user for the sacct command".to_string())?;

        let num_days: i64 = if let Some(days) = args.get("days") {
            days.parse().map_err(|_| {
                "Provided number of days could not be formatted into number".to_string()
            })?
        } else {
            100
        };

        let start_time = (Utc::now() - Duration::days(num_days))
            .format("%Y-%m-%d")
            .to_string();

        let output = Command::new("sacct")
            .args(["--user", user])
            .args(["--starttime", &start_time])
            .arg("--json")
            .output()
            .map_err(|_| {
                "Failed to run sinfo command internally, consider piping it in".to_string()
            })?;

        result_string = String::from_utf8_lossy(&output.stdout).to_string();

        let structure: SacctData = json_string_to_struct(result_string).map_err(|_| {
            "Faield to format input into internal structure properly - consider piping it in"
                .to_string()
        })?;

        Ok(StructOptions::Sacct(structure))
    }
}
