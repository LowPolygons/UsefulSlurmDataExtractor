use crate::{containers::slurm_data::SlurmJob, systems::filter::Filterable};

pub struct JobStatusFilter {
    status: Vec<String>,
}

impl JobStatusFilter {
    pub fn new(status: Vec<String>) -> Self {
        JobStatusFilter { status }
    }
}

impl Filterable for JobStatusFilter {
    fn does_job_meet_filter_reqs(&self, job: &SlurmJob) -> bool {
        todo!()
    }
}
