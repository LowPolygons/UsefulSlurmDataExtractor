use crate::{containers::slurm_data::SlurmJob, systems::filter::Filterable};

pub struct DirectoryFilter {
    directories: Vec<String>,
}

impl DirectoryFilter {
    pub fn new(directories: Vec<String>) -> Self {
        DirectoryFilter { directories }
    }
}

impl Filterable for DirectoryFilter {
    fn does_job_meet_filter_reqs(&self, job: &SlurmJob) -> bool {
        todo!()
    }
}
