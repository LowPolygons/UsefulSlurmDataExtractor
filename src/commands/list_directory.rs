use std::path::Path;

use crate::{
    cli::FilterOptions,
    commands::{command::CommandCall, get_job_selection_through_menu},
    containers::slurm_data::{SlurmData, SlurmJob},
    systems::filter::print_help_filter_info,
    utils::{
        filtered_data_from_list::filtered_data_from_list,
        print_working_directory::print_working_directory,
    },
};

pub struct ListDirectory {
    pub filter: Option<FilterOptions>,
    pub values: Vec<String>,
}

impl CommandCall for ListDirectory {
    fn command(&self, structure: &SlurmData) -> Result<(), ()> {
        let filtered_data: Vec<SlurmJob> =
            filtered_data_from_list(structure, &self.filter, &self.values);

        let default_options: Vec<String> = vec![String::from("Finish")];

        let selection: usize = get_job_selection_through_menu(&filtered_data, default_options)
            .map_err(|e| {
                println!("Get job menu failure: {e}");
                return ();
            })?;

        if selection == 0 {
            if filtered_data.len() == 0
                && structure.jobs.len() != 0
                && let Some(filter_choice) = &self.filter
            {
                print_help_filter_info(&structure.jobs, &filter_choice);
            }
            return Ok(());
        }

        let working_directory = Path::new(&filtered_data[selection - 1].current_working_directory);

        print_working_directory(working_directory, false).map_err(|e| {
            println!("Print working directory failure: {e}");
            return ();
        })?;

        Ok(())
    }
}
