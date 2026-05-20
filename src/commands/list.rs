use crate::{
    cli::FilterOptions,
    containers::slurm_data::SlurmData,
    utils::{
        filtered_data_from_list::filtered_data_from_list,
        print_common_job_info::print_common_job_info,
    },
};

pub fn command(
    slurm_data: &SlurmData,
    filter: &Option<FilterOptions>,
    values: &Vec<String>,
) -> Result<(), ()> {
    let filtered_data = filtered_data_from_list(slurm_data, filter, values);

    filtered_data
        .iter()
        .try_for_each(|job_data| -> Result<(), ()> {
            println!("==========================");
            print_common_job_info(job_data).map_err(|e| {
                println!("Error printing job info: {e}");
                return ();
            })?;
            Ok(())
        })
        .map_err(|_| ())?;

    println!("==========================");
    println!("Listed info for {} jobs", filtered_data.len());
    println!("==========================");

    Ok(())
}
