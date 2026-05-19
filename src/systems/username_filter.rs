use crate::{containers::slurm_data::SlurmJob, systems::filter::Filterable};

pub struct UsernameFilter {
    usernames: Vec<String>,
}

impl UsernameFilter {
    pub fn new(usernames: Vec<String>) -> Self {
        UsernameFilter { usernames }
    }
}

impl Filterable for UsernameFilter {
    fn does_job_meet_filter_reqs(&self, job: &SlurmJob) -> bool {
        self.usernames.contains(&job.user_name)
    }
}
