use crate::{
    cli::FilterOptions,
    commands::command::CommandCall,
    containers::{
        piped_input::{PipedInputHandler, StructOptions},
        slurm_data::SlurmData,
        slurm_handler::SlurmHandler,
    },
    systems::filter::print_help_filter_info,
    utils::{
        filtered_data_from_list::filtered_data_from_list,
        print_common_job_info::print_common_job_info,
    },
};

pub struct List {
    pub filter: Option<FilterOptions>,
    pub values: Vec<String>,
}

impl CommandCall for List {
    fn command(&self, slurm_data: &StructOptions) -> Result<(), ()> {
        let matched_struct: &SlurmData = match slurm_data {
            StructOptions::Slurm(slurm_data) => slurm_data,
            StructOptions::Sacct(sacct_data) => return Err(()),
            StructOptions::Sinfo(sinfo_data) => return Err(()),
        };

        let filtered_data = filtered_data_from_list(matched_struct, &self.filter, &self.values);

        filtered_data
            .iter()
            .try_for_each(|job_data| -> Result<(), ()> {
                println!("==========================");
                print_common_job_info(job_data).map_err(|e| {
                    println!("Error printing job info: {e}");
                    return ();
                })?;
                Ok(())
            })
            .map_err(|_| ())?;

        println!("==========================");
        println!("Listed info for {} jobs", filtered_data.len());
        println!("==========================");

        if filtered_data.len() == 0
            && matched_struct.jobs.len() != 0
            && let Some(filter_choice) = &self.filter
        {
            print_help_filter_info(&matched_struct.jobs, &filter_choice);
        }
        Ok(())
    }

    fn get_piped_input_handler(&self) -> Box<dyn PipedInputHandler> {
        return Box::new(SlurmHandler::new());
    }
}
