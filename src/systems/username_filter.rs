use crate::systems::filter::{ExtractsFilterableCategories, Filterable};

pub struct UsernameFilter {
    usernames: Vec<String>,
}

impl UsernameFilter {
    pub fn new(usernames: Vec<String>) -> Self {
        UsernameFilter { usernames }
    }
}

impl Filterable for UsernameFilter {
    fn does_job_meet_filter_reqs(&self, job: &dyn ExtractsFilterableCategories) -> bool {
        self.usernames.contains(&job.get_username())
    }
}
