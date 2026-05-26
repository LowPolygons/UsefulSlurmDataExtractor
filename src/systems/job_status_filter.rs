use crate::systems::filter::{ExtractsFilterableCategories, Filterable};

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

    // fn print_help_text(&self, all_jobs: &Vec<Box<dyn ExtractsFilterableCategories>>) {
    //     let mut all_status: Vec<String> = vec![];
    //
    //     all_jobs.iter().for_each(|job| {
    //         if !all_status.contains(&job.get_account()) {
    //             all_status.push(job.get_account())
    //         }
    //     });
    //
    //     println!("The full list of jobs has this range of 'status':");
    //     all_status.iter().for_each(|x| println!("- {x}"));
    // }
}
