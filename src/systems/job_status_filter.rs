use crate::{
    containers::slurm_data::SlurmJob,
    systems::filter::{ExtractsFilterableCategories, Filterable},
};

pub struct JobStatusFilter {
    status: Vec<String>,
}

impl JobStatusFilter {
    pub fn new(status: Vec<String>) -> Self {
        JobStatusFilter { status }
    }
}

impl Filterable for JobStatusFilter {
    fn does_job_meet_filter_reqs(&self, job: &dyn ExtractsFilterableCategories) -> bool {
        self.status.contains(&job.get_job_status())
    }
}
