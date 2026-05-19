use std::path::Path;

use dialoguer::{Select, theme::ColorfulTheme};

use crate::{
    cli::FilterOptions,
    containers::slurm_data::{SlurmData, SlurmJob},
    utils::{
        filtered_data_from_list::filtered_data_from_list,
        print_working_directory::print_working_directory,
    },
};

// TODO: Correct repeated code in this and detail or any that uses a selection list
pub fn command(
    structure: &SlurmData,
    filter: &Option<FilterOptions>,
    values: &Vec<String>,
) -> Result<(), ()> {
    let filtered_data: Vec<SlurmJob> = filtered_data_from_list(structure, filter, values);

    let mut selection_info: Vec<String> = vec![String::from("Finish")];

    selection_info = filtered_data.iter().fold(selection_info, |mut vec, job| {
        vec.push(format!(
            "Name and ID: {}, {} | User Name: {} | Status: {}",
            job.name, job.job_id, job.user_name, job.job_state
        ));

        vec
    });
    // loop {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a job to view the working directory of")
        .items(&selection_info)
        .default(0)
        .interact()
        .map_err(|_| ())?;

    if selection == 0 {
        return Ok(());
    }

    let working_directory = Path::new(&filtered_data[selection - 1].current_working_directory);

    print_working_directory(working_directory, false).map_err(|e| {
        println!("{e}");
        return ();
    })?;

    Ok(())
}
