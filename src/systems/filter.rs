use crate::{
    cli::FilterOptions,
    containers::slurm_data::SlurmJob,
    systems::{
        account_filter::AccountFilter, directory_filter::DirectoryFilter,
        job_name_filter::JobNameFilter, job_status_filter::JobStatusFilter,
        num_nodes_filter::NumNodesFilter, username_filter::UsernameFilter,
    },
};

pub trait Filterable {
    fn does_job_meet_filter_reqs(&self, job: &SlurmJob) -> bool;
}

pub fn get_filter_object(
    options: &FilterOptions,
    values: Vec<String>,
) -> Option<Box<dyn Filterable>> {
    return match options {
        FilterOptions::Directory => Some(Box::new(DirectoryFilter::new(values))),
        FilterOptions::Name => Some(Box::new(JobNameFilter::new(values))),
        FilterOptions::JobStatus => Some(Box::new(JobStatusFilter::new(values))),
        FilterOptions::NumNodes => match NumNodesFilter::new(values) {
            Some(v) => Some(Box::new(v)),
            None => None,
        },
        FilterOptions::Account => Some(Box::new(AccountFilter::new(values))),
        FilterOptions::Username => Some(Box::new(UsernameFilter::new(values))),
        FilterOptions::None => None,
    };
}
