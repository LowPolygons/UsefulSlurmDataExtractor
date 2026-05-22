use std::{env, path::Path};

use chrono::DateTime;
use dialoguer::{Select, theme::ColorfulTheme};

use crate::{
    cli::FilterOptions,
    commands::command::CommandCall,
    containers::slurm_data::{SlurmData, SlurmJob},
    utils::filtered_data_from_list::filtered_data_from_list,
};

pub struct CancelHelp {
    pub filter: Option<FilterOptions>,
    pub values: Vec<String>,
}

impl CommandCall for CancelHelp {
    fn command(&self, structure: &SlurmData) -> Result<(), ()> {
        let filtered_data: Vec<SlurmJob> =
            filtered_data_from_list(structure, &self.filter, &self.values);

        let mut job_ids_to_cancel: Vec<u64> = vec![];

        let mut selection_info: Vec<String> =
            vec![String::from("Finish"), String::from("Clear list")];

        selection_info = filtered_data.iter().fold(selection_info, |mut vec, job| {
            vec.push(format!(
                "Name and ID: {}, {} | Directory: {} | Status: {} | Submit Time: {}",
                job.name,
                job.job_id,
                job.current_working_directory,
                job.job_state,
                DateTime::from_timestamp(job.submit_time as i64, 0).unwrap_or(DateTime::default())
            ));

            vec
        });

        loop {
            let list = job_ids_to_cancel
                .iter()
                .fold(String::from(""), |mut string, j| {
                    string = format!("{} {}", string, j);

                    string
                });

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "Choose a job to cancel - Current IDs are: {}",
                    list
                ))
                .items(&selection_info)
                .default(0)
                .interact()
                .map_err(|e| {
                    println!("Error in selection: {e}");
                    return ();
                })?;

            if selection == 0 {
                break;
            } else if selection == 1 {
                job_ids_to_cancel = Vec::new();
            } else {
                job_ids_to_cancel.push(filtered_data[selection - 2].job_id);

                println!("Job with ID {} added", filtered_data[selection - 2].job_id);
            }
        }

        if job_ids_to_cancel.len() > 0 {
            std::fs::write(
        "slurm_helper_cancel_script.sh",
        job_ids_to_cancel
            .iter()
            .fold(
                vec![format!(
                    "rm {}/slurm_helper_cancel_script.sh",
                    env::current_dir().map_err(|_| {
                        println!("Failure: Could not get the current directory for writing the cancel script");
                        return ()
                    })?.display()
                )],
                |mut vec, id| {
                    vec.push(format!("scancel {id}\n"));
                    vec
                },
            )
            .join("\n"),
        )
        .map_err(|_| {
            println!("Failed to write cancel script. Your list of jobs you wanted to cancel was: ");

            job_ids_to_cancel.iter().for_each(|j| println!("- {j}"));

            ()
        })?;

            println!("Wrote the file 'slurm_helper_cancel_script.sh' to your current directory.");
        } else {
            println!("No jobs chosen to cancel.");
        }
        Ok(())
    }
}
