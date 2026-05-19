use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use chrono::DateTime;
use dialoguer::{Select, theme::ColorfulTheme};

use crate::{
    cli::FilterOptions,
    containers::slurm_data::{SlurmData, SlurmJob},
    utils::{
        filtered_data_from_list::filtered_data_from_list,
        print_common_job_info::print_common_job_info, secs_to_nice_time::secs_to_nice_time,
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

    let mut selection_info: Vec<String> = vec![String::from("Finish")];

    selection_info = filtered_data.iter().fold(selection_info, |mut vec, job| {
        vec.push(format!(
            "Name and ID: {}, {} | User Name: {} | Status: {}",
            job.name, job.job_id, job.user_name, job.job_state
        ));

        vec
    });
    // loop {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a job to view in more detail")
        .items(&selection_info)
        .default(0)
        .interact()
        .map_err(|_| ())?;

    if selection == 0 {
        return Ok(());
    }

    print_infomation_about_file(&filtered_data[selection - 1]).map_err(|e| {
        println!("Error: {e}");
        return ();
    })?;
    // }
    Ok(())
}

fn print_infomation_about_file(target_job: &SlurmJob) -> Result<(), String> {
    println!("==========================");
    print_common_job_info(target_job)?;

    println!("--------------------------");
    println!("Files in working directory:");
    let working_directory = Path::new(&target_job.current_working_directory);
    print_working_directory(working_directory)?;
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

fn print_working_directory(working_directory: &Path) -> Result<(), String> {
    if working_directory
        .try_exists()
        .map_err(|_| String::from("Couldn't determine if working directory exists"))?
        && working_directory.is_dir()
    {
        let mut elements_in_dir = working_directory
            .read_dir()
            .map_err(|_| String::from("Failed to get elements in directory"))?;

        elements_in_dir.try_for_each(|elem| -> Result<(), String> {
            let path = elem
                .map_err(|_| String::from("Bad element in directory"))?
                .path();

            if path.is_dir() {
                println!("..Dir.. {:?}", path);
            } else {
                println!("..File.. {:?}", path);
            }
            Ok(())
        })?;
    } else {
        println!("Couldn't find working directory");
    }
    Ok(())
}

fn try_print_any_output_file(file: &Path, target_job: &SlurmJob) -> Result<(), String> {
    if file
        .try_exists()
        .map_err(|_| String::from("Couldn't determine if output file exists"))?
    {
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

fn line_vec_from_file(file_name: &str) -> Result<Vec<String>, String> {
    let file: File = File::open(file_name).map_err(|e| e.to_string())?;

    return Ok(io::BufReader::new(file).lines().into_iter().try_fold(
        Vec::<String>::new(),
        |mut vec, line| -> Result<Vec<String>, String> {
            vec.push(String::from(line.map_err(|e| e.to_string())?));

            return Ok(vec);
        },
    )?);
}
