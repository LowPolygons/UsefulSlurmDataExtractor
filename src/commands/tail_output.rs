use std::{cmp::min, path::Path};

use crate::{
    cli::FilterOptions,
    commands::{command::CommandCall, get_job_selection_through_menu, line_vec_from_file},
    containers::{
        piped_input::{PipedInputHandler, StructOptions},
        slurm_data::{SlurmData, SlurmJob},
        slurm_handler::SlurmHandler,
    },
    systems::filter::print_help_filter_info,
    utils::filtered_data_from_list::filtered_data_from_list,
};

pub struct TailOutput {
    pub filter: Option<FilterOptions>,
    pub values: Vec<String>,
    pub num_lines: Option<u8>,
}

impl CommandCall for TailOutput {
    fn command(&self, slurm_data: &StructOptions) -> Result<(), ()> {
        let structure: &SlurmData = match slurm_data {
            StructOptions::Slurm(slurm_data) => slurm_data,
            StructOptions::Sacct(sacct_data) => return Err(()),
            StructOptions::Sinfo(sinfo_data) => return Err(()),
        };
        let filtered_data: Vec<SlurmJob> =
            filtered_data_from_list(structure, &self.filter, &self.values)
                .into_iter()
                .filter(|job| job.job_state != "PENDING")
                .collect();

        if filtered_data.len() > 0 {
            let selection: usize =
                get_job_selection_through_menu(&filtered_data, Vec::new()).map_err(|_| ())?;

            let output_file = Path::new(&filtered_data[selection].standard_output);

            if output_file.try_exists().map_err(|e| {
                println!(
                    "Couldn't validate if the output file exists: {}",
                    e.to_string()
                );
                return ();
            })? {
                let lines =
                    line_vec_from_file(&filtered_data[selection].standard_output).map_err(|e| {
                        println!("Line vec from file error: {e}");
                        return ();
                    })?;

                // Clamp so there are no underflows
                let num_lines_to_show: usize =
                    min(self.num_lines.unwrap_or(30).into(), lines.len());

                println!("------------------------");
                println!("The last {} lines of the output file: ", num_lines_to_show);
                lines.iter().enumerate().for_each(|(index, line)| {
                    if index > (lines.len() - num_lines_to_show) {
                        println!(".. {}", line);
                    }
                });
            } else {
                println!("Could not find the output file");
            }
        }

        if filtered_data.len() == 0
            && structure.jobs.len() != 0
            && let Some(filter_choice) = &self.filter
        {
            print_help_filter_info(&structure.jobs, &filter_choice);
        }

        Ok(())
    }

    fn get_piped_input_handler(&self) -> Box<dyn PipedInputHandler> {
        return Box::new(SlurmHandler::new());
    }
}
