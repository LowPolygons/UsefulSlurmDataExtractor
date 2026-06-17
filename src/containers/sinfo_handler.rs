use std::process::Command;

use crate::{
    cli::Commands,
    containers::{piped_input::PipedInputHandler, sinfo_data::SinfoData},
    utils::json_string_to_struct::json_string_to_struct,
};

use super::piped_input::StructOptions;

pub struct SinfoHandler {}

impl SinfoHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl PipedInputHandler for SinfoHandler {
    fn try_make_piped_input_into_struct(&self, input: String) -> Result<StructOptions, String> {
        let structure: SinfoData = json_string_to_struct(input).map_err(|_| {
            "Piped input could not be mapped to struct - did you pipe it in correctly?".to_string()
        })?;

        Ok(StructOptions::Sinfo(structure))
    }

    fn try_run_command_to_get_struct(
        &self,
        _expects_no_args: std::collections::HashMap<String, String>,
    ) -> Result<StructOptions, String> {
        let result_string: String;

        let output = Command::new("sinfo").arg("--json").output().map_err(|_| {
            "Failed to run sinfo command internally, consider piping it in".to_string()
        })?;

        result_string = String::from_utf8_lossy(&output.stdout).to_string();

        let structure: SinfoData = json_string_to_struct(result_string).map_err(|_| {
            "Faield to format input into internal structure properly - consider piping it in"
                .to_string()
        })?;

        Ok(StructOptions::Sinfo(structure))
    }
}
