use std::{path::Path, process::Command};

use dialoguer::{Select, theme::ColorfulTheme};

use crate::{
    cli::FilterOptions,
    commands::{get_job_selection_through_menu, line_vec_from_file},
    containers::slurm_data::{SlurmData, SlurmJob},
    utils::{
        filtered_data_from_list::filtered_data_from_list,
        print_common_job_info::print_common_job_info,
        print_working_directory::print_working_directory,
    },
};

pub fn command(
    structure: &SlurmData,
    job_id: &Option<u64>,
    filter: &Option<FilterOptions>,
    values: &Vec<String>,
) -> Result<(), ()> {
    if let Some(id) = job_id {
        let job_ids = structure
            .jobs
            .iter()
            .fold(Vec::<u64>::new(), |mut vec, job| {
                vec.push(job.job_id);
                return vec;
            });
        if job_ids.contains(id) {
            let target_job: &SlurmJob = &structure.jobs[job_ids
                .iter()
                .position(|&item| item.eq(id))
                .unwrap_or(job_ids.len() + 1)];

            print_infomation_about_file(target_job).map_err(|e| {
                println!("Error: {}", e);
                return ();
            })?;
        }
        return Ok(());
    }
    let filtered_data: Vec<SlurmJob> = filtered_data_from_list(structure, filter, values);

    loop {
        let default_options: Vec<String> = vec![String::from("Finish")];
        let selection: usize =
            get_job_selection_through_menu(&filtered_data, default_options).map_err(|_| ())?;

        if selection == 0 {
            return Ok(());
        }

        print_infomation_about_file(&filtered_data[selection - 1]).map_err(|e| {
            println!("Error: {e}");
            return ();
        })?;

        let options = vec!["Back", "Cancel Job"];
        let inner_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you wish to cancel the job?")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|_| ())?;

        if inner_selection == 0 {
            continue;
        } else {
            println!(
                "Cancelling Job with ID {}..",
                filtered_data[selection - 1].job_id
            );
            Command::new(format!("scancel {}", filtered_data[selection - 1].job_id))
                .output()
                .map_err(|_| {
                    println!("Failed to execute the cancel command");
                    return ();
                })?;
        }
    }
}

fn print_infomation_about_file(target_job: &SlurmJob) -> Result<(), String> {
    println!("==========================");
    print_common_job_info(target_job)?;

    println!("--------------------------");

    println!("Files in working directory:");
    let working_directory = Path::new(&target_job.current_working_directory);
    print_working_directory(working_directory, true)?;

    println!("--------------------------");

    println!(
        "Job max length: {} hours",
        target_job.time_limit.number / 60.0
    );
    println!("Number of nodes: {}", target_job.node_count.number);
    println!(
        "Number of tasks per node: {}",
        target_job.tasks_per_node.number
    );

    println!("--------------------------");
    // Output file if it exists
    let output_file = Path::new(&target_job.standard_output);
    let error_file = Path::new(&target_job.standard_error);

    println!("Output File: {}", target_job.standard_output);
    try_print_any_output_file(output_file, target_job)
        .map_err(|e| format!("Error printing ouput file: {e}"))?;

    if target_job.standard_error == target_job.standard_output {
        return Ok(());
    }

    println!("--------------------------");

    println!("Error File: {}", target_job.standard_error);
    try_print_any_output_file(error_file, target_job)
        .map_err(|e| format!("Error printing error file: {e}"))?;

    Ok(())
}

fn try_print_any_output_file(file: &Path, target_job: &SlurmJob) -> Result<(), String> {
    if file.try_exists().map_err(|_| {
        String::from("Couldn't determine if output file exists. You may not have access rights")
    })? {
        let lines = line_vec_from_file(&target_job.standard_output).map_err(|e| {
            return e;
        })?;

        let mut num_hidden: i32 = 0;

        lines.iter().enumerate().for_each(|(index, line)| {
            if index < 5 || index > lines.len() - 5 {
                println!("{}", line);
            } else {
                num_hidden = num_hidden + 1;

                if index == lines.len() - 6 {
                    println!(".. {} lines hidden ..", num_hidden);
                }
            }
        });
    } else {
        println!("No file found");
    }
    println!("--------------------------");

    Ok(())
}
