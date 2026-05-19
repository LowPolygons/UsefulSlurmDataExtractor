use crate::{
    cli::FilterOptions,
    containers::slurm_data::{SlurmData, SlurmJob},
    systems::filter::{self, get_filter_object},
};

pub fn filtered_data_from_list(
    slurm_data: &SlurmData,
    filter: &Option<FilterOptions>,
    values: &Vec<String>,
) -> Vec<SlurmJob> {
    let filtered_data = match filter {
        Some(filter_option) => {
            if let Some(filter) = get_filter_object(filter_option, values.clone()) {
                slurm_data
                    .jobs
                    .clone()
                    .into_iter()
                    .filter(|job| filter.does_job_meet_filter_reqs(job))
                    .collect()
            } else {
                slurm_data.jobs.clone()
            }
        }
        None => slurm_data.jobs.clone(),
    };

    return filtered_data;
}
