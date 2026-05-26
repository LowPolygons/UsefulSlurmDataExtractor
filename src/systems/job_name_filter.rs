use crate::systems::filter::{ExtractsFilterableCategories, Filterable};

pub struct JobNameFilter {
    name_contains: Vec<String>,
}

impl JobNameFilter {
    pub fn new(name_contains: Vec<String>) -> Self {
        JobNameFilter { name_contains }
    }
}

impl Filterable for JobNameFilter {
    fn does_job_meet_filter_reqs(&self, job: &dyn ExtractsFilterableCategories) -> bool {
        let mut success: bool = false;
        self.name_contains.iter().for_each(|dir| {
            if job.get_name().contains(dir) {
                success = true;
            }
        });

        success
    }

    // fn print_help_text(&self, all_jobs: &Vec<Box<dyn ExtractsFilterableCategories>>) {
    //     // Not helpful for job name
    //     // Its technically a violation of ISP but its so insubstantially small
    // }
}
