use std::process::Command;

use crate::{
    containers::{piped_input::PipedInputHandler, slurm_data::SlurmData},
    utils::json_string_to_struct::json_string_to_struct,
};

use super::piped_input::StructOptions;

pub struct SlurmHandler {}

impl SlurmHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl PipedInputHandler for SlurmHandler {
    fn try_make_piped_input_into_struct(&self, input: String) -> Result<StructOptions, String> {
        let structure: SlurmData = json_string_to_struct(input).map_err(|_| {
            "Piped input could not be mapped to struct - did you pipe it in correctly?".to_string()
        })?;

        Ok(StructOptions::Slurm(structure))
    }

    fn try_run_command_to_get_struct(
        &self,
        args: std::collections::HashMap<String, String>,
    ) -> Result<StructOptions, String> {
        let result_string: String;

        let requires_all_jobs: bool = args.contains_key("requires_all_in_queue");

        let output = if requires_all_jobs {
            Command::new("squeue").arg("--json").output()
        } else {
            Command::new("squeue").arg("--m").arg("--json").output()
        }
        .map_err(|_| {
            "Failed to run squeue command internally, consider piping it in".to_string()
        })?;

        result_string = String::from_utf8_lossy(&output.stdout).to_string();

        let structure: SlurmData = json_string_to_struct(result_string).map_err(|_| {
            "Faield to format input into internal structure properly - consider piping it in"
                .to_string()
        })?;

        Ok(StructOptions::Slurm(structure))
    }
}
