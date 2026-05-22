use std::{cmp::min, path::Path};

use crate::{
    cli::FilterOptions,
    commands::{command::CommandCall, get_job_selection_through_menu, line_vec_from_file},
    containers::slurm_data::{SlurmData, SlurmJob},
    utils::filtered_data_from_list::filtered_data_from_list,
};

pub struct TailOutput {
    pub filter: Option<FilterOptions>,
    pub values: Vec<String>,
    pub num_lines: Option<u8>,
}

impl CommandCall for TailOutput {
    fn command(&self, structure: &SlurmData) -> Result<(), ()> {
        let filtered_data: Vec<SlurmJob> =
            filtered_data_from_list(structure, &self.filter, &self.values)
                .into_iter()
                .filter(|job| job.job_state != "PENDING")
                .collect();

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
            let num_lines_to_show: usize = min(self.num_lines.unwrap_or(30).into(), lines.len());

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

        Ok(())
    }
}
