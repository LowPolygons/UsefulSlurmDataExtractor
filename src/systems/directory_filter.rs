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
        let mut success: bool = false;
        self.directories.iter().for_each(|dir| {
            if job.current_working_directory.contains(dir) {
                success = true;
            }
        });

        success
    }
}
