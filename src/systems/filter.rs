use crate::{
    cli::FilterOptions,
    systems::{
        account_filter::AccountFilter, directory_filter::DirectoryFilter,
        job_name_filter::JobNameFilter, job_status_filter::JobStatusFilter,
        num_nodes_filter::NumNodesFilter, username_filter::UsernameFilter,
    },
};

pub trait ExtractsFilterableCategories {
    fn get_directory(&self) -> String;
    fn get_name(&self) -> String;
    fn get_job_status(&self) -> String;
    fn get_num_nodes(&self) -> u16;
    fn get_account(&self) -> String;
    fn get_username(&self) -> String;
}

pub trait Filterable {
    fn does_job_meet_filter_reqs(&self, job: &dyn ExtractsFilterableCategories) -> bool;
}

pub fn get_filter_object(
    options: &FilterOptions,
    values: Vec<String>,
) -> Option<Box<dyn Filterable>> {
    return match options {
        FilterOptions::Directory => Some(Box::new(DirectoryFilter::new(values))),
        FilterOptions::Name => Some(Box::new(JobNameFilter::new(values))),
        FilterOptions::JobStatus => Some(Box::new(JobStatusFilter::new(values))),
        FilterOptions::NumNodes => match NumNodesFilter::new(values) {
            Some(v) => Some(Box::new(v)),
            None => None,
        },
        FilterOptions::Account => Some(Box::new(AccountFilter::new(values))),
        FilterOptions::Username => Some(Box::new(UsernameFilter::new(values))),
        FilterOptions::None => None,
    };
}

pub fn print_help_filter_info(
    all_jobs: &Vec<impl ExtractsFilterableCategories>,
    category: &FilterOptions,
) {
    match category {
        FilterOptions::Account => {
            let mut all_accounts: Vec<String> = vec![];

            all_jobs.iter().for_each(|job| {
                if !all_accounts.contains(&job.get_account()) {
                    all_accounts.push(job.get_account())
                }
            });

            println!("The full list of jobs has this range of 'accounts':");
            all_accounts.iter().for_each(|x| println!("- {x}"));
        }
        FilterOptions::JobStatus => {
            let mut all_status: Vec<String> = vec![];

            all_jobs.iter().for_each(|job| {
                if !all_status.contains(&job.get_account()) {
                    all_status.push(job.get_account())
                }
            });

            println!("The full list of jobs has this range of 'status':");
            all_status.iter().for_each(|x| println!("- {x}"));
        }
        FilterOptions::Username => {
            let mut all_usernames: Vec<String> = vec![];

            all_jobs.iter().for_each(|job| {
                if !all_usernames.contains(&job.get_account()) {
                    all_usernames.push(job.get_account())
                }
            });

            println!("The full list of jobs has this range of 'usernames':");
            all_usernames.iter().for_each(|x| println!("- {x}"));
        }
        _ => {}
    }
}
