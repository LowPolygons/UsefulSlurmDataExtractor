use crate::{containers::slurm_data::SlurmJob, systems::filter::Filterable};

pub struct JobNameFilter {
    name_contains: Vec<String>,
}

impl JobNameFilter {
    pub fn new(name_contains: Vec<String>) -> Self {
        JobNameFilter { name_contains }
    }
}

impl Filterable for JobNameFilter {
    fn does_job_meet_filter_reqs(&self, job: &SlurmJob) -> bool {
        todo!()
    }
}
