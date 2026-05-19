use std::path::Path;

use dialoguer::{Select, theme::ColorfulTheme};

use crate::{
    cli::FilterOptions,
    commands::line_vec_from_file,
    containers::slurm_data::{SlurmData, SlurmJob},
    utils::filtered_data_from_list::filtered_data_from_list,
};

pub fn command(
    structure: &SlurmData,
    filter: &Option<FilterOptions>,
    values: &Vec<String>,
    num_lines: &Option<u8>,
) -> Result<(), ()> {
    let filtered_data: Vec<SlurmJob> = filtered_data_from_list(structure, filter, values)
        .into_iter()
        .filter(|job| job.job_state != "PENDING")
        .collect();

    let mut selection_info: Vec<String> = vec![];

    selection_info = filtered_data.iter().fold(selection_info, |mut vec, job| {
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
        .map_err(|_| ())?;

    let output_file = Path::new(&filtered_data[selection].standard_output);

    let num_lines_to_show: usize = num_lines.unwrap_or(30).into();

    if output_file.try_exists().map_err(|_| {
        println!("Couldn't validate if the output file exists");
        return ();
    })? {
        let lines = line_vec_from_file(&filtered_data[selection].standard_output).map_err(|e| {
            println!("{e}");
            return ();
        })?;

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
