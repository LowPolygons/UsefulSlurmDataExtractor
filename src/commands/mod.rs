use std::{
    fs::File,
    io::{self, BufRead},
};

use dialoguer::{Select, theme::ColorfulTheme};

use crate::containers::slurm_data::SlurmJob;

pub mod cancel_help;
pub mod command;
pub mod detail;
pub mod list;
pub mod list_directory;
pub mod sinfo;
pub mod system_capacity;
pub mod tail_output;

pub fn line_vec_from_file(file_name: &str) -> Result<Vec<String>, String> {
    let file: File = File::open(file_name).map_err(|e| e.to_string())?;

    return Ok(io::BufReader::new(file).lines().into_iter().try_fold(
        Vec::<String>::new(),
        |mut vec, line| -> Result<Vec<String>, String> {
            vec.push(String::from(line.map_err(|e| e.to_string())?));

            return Ok(vec);
        },
    )?);
}

pub fn get_job_selection_through_menu(
    jobs: &Vec<SlurmJob>,
    hardcoded_first_options: Vec<String>,
) -> Result<usize, String> {
    let selection_info = jobs.iter().fold(hardcoded_first_options, |mut vec, job| {
        vec.push(format!(
            "Name and ID: {}, {} | User Name: {} | Status: {}",
            job.name, job.job_id, job.user_name, job.job_state
        ));
        vec
    });

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a job to view the tail of the output")
        .items(&selection_info)
        .default(0)
        .interact()
        .map_err(|_| String::from("Interative Menu Failure"))?;

    return Ok(selection);
}
