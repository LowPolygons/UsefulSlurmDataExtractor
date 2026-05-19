use crate::{
    containers::slurm_data::SlurmJob,
    systems::filter::Filterable,
};

pub struct AccountFilter {
    account_names: Vec<String>,
}

impl AccountFilter {
    pub fn new(account_names: Vec<String>) -> Self {
        AccountFilter { account_names }
    }
}

impl Filterable for AccountFilter {
    fn does_job_meet_filter_reqs(&self, job: &SlurmJob) -> bool {
        self.account_names.contains(&job.account)
    }
}
