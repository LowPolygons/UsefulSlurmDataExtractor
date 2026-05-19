use std::path::Path;


use crate::{
    cli::FilterOptions,
    commands::get_job_selection_through_menu,
    containers::slurm_data::{SlurmData, SlurmJob},
    utils::{
        filtered_data_from_list::filtered_data_from_list,
        print_working_directory::print_working_directory,
    },
};

pub fn command(
    structure: &SlurmData,
    filter: &Option<FilterOptions>,
    values: &Vec<String>,
) -> Result<(), ()> {
    let filtered_data: Vec<SlurmJob> = filtered_data_from_list(structure, filter, values);

    let default_options: Vec<String> = vec![String::from("Finish")];

    let selection: usize =
        get_job_selection_through_menu(&filtered_data, default_options).map_err(|_| ())?;

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
