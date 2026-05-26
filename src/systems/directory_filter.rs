use crate::systems::filter::{ExtractsFilterableCategories, Filterable};

pub struct DirectoryFilter {
    directories: Vec<String>,
}

impl DirectoryFilter {
    pub fn new(directories: Vec<String>) -> Self {
        DirectoryFilter { directories }
    }
}

impl Filterable for DirectoryFilter {
    fn does_job_meet_filter_reqs(&self, job: &dyn ExtractsFilterableCategories) -> bool {
        let mut success: bool = false;
        self.directories.iter().for_each(|dir| {
            if job.get_directory().contains(dir) {
                success = true;
            }
        });

        success
    }
}
