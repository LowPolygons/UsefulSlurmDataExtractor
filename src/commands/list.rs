use crate::{
    cli::FilterOptions,
    commands::command::CommandCall,
    containers::slurm_data::SlurmData,
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
    fn command(&self, slurm_data: &SlurmData) -> Result<(), ()> {
        let filtered_data = filtered_data_from_list(slurm_data, &self.filter, &self.values);

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
            && slurm_data.jobs.len() != 0
            && let Some(filter_choice) = &self.filter
        {
            print_help_filter_info(&slurm_data.jobs, &filter_choice);
        }
        Ok(())
    }
}
