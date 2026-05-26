use crate::systems::filter::{ExtractsFilterableCategories, Filterable};

pub struct AccountFilter {
    account_names: Vec<String>,
}

impl AccountFilter {
    pub fn new(account_names: Vec<String>) -> Self {
        AccountFilter { account_names }
    }
}

impl Filterable for AccountFilter {
    fn does_job_meet_filter_reqs(&self, job: &dyn ExtractsFilterableCategories) -> bool {
        self.account_names.contains(&job.get_account())
    }

    // fn print_help_text(&self, all_jobs: &Vec<Box<dyn ExtractsFilterableCategories>>) {
    //     let mut all_accounts: Vec<String> = vec![];
    //
    //     all_jobs.iter().for_each(|job| {
    //         if !all_accounts.contains(&job.get_account()) {
    //             all_accounts.push(job.get_account())
    //         }
    //     });
    //
    //     println!("The full list of jobs has this range of 'accounts':");
    //     all_accounts.iter().for_each(|x| println!("- {x}"));
    // }
}
