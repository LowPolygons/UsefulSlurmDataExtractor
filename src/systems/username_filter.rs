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

    // fn print_help_text(&self, all_jobs: &Vec<Box<dyn ExtractsFilterableCategories>>) {
    //     let mut all_usernames: Vec<String> = vec![];
    //
    //     all_jobs.iter().for_each(|job| {
    //         if !all_usernames.contains(&job.get_account()) {
    //             all_usernames.push(job.get_account())
    //         }
    //     });
    //
    //     println!("The full list of jobs has this range of 'usernames':");
    //     all_usernames.iter().for_each(|x| println!("- {x}"));
    // }
}
