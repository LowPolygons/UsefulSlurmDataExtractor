use crate::{
    containers::slurm_data::SlurmJob,
    systems::{
        account_filter::AccountFilter, directory_filter::DirectoryFilter,
        job_name_filter::JobNameFilter, job_status_filter::JobStatusFilter,
        num_nodes_filter::NumNodesFilter, username_filter::UsernameFilter,
    },
};

pub enum FilterOptions {
    Directory(DirectoryFilter),
    JobName(JobNameFilter),
    JobStatus(JobStatusFilter),
    NumNodes(NumNodesFilter),
    Account(AccountFilter),
    Username(UsernameFilter),
}

pub trait Filterable {
    fn does_job_meet_filter_reqs(&self, job: &SlurmJob) -> bool;
}
