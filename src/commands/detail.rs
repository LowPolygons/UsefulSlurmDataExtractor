use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use chrono::DateTime;

use crate::{
    cli::FilterOptions,
    containers::slurm_data::{SlurmData, SlurmJob},
};

pub fn command(
    structure: &SlurmData,
    job_id: &Option<u64>,
    filter: &FilterOptions,
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
    }
    Ok(())
}

fn print_infomation_about_file(target_job: &SlurmJob) -> Result<(), String> {
    println!("==========================");
    println!("Job Name & ID: {}, {}", target_job.name, target_job.job_id);
    println!(
        "User Name and ID: {}, {}",
        target_job.user_name, target_job.user_id
    );
    println!("--------------------------");
    println!(
        "Submit Time: {}",
        DateTime::from_timestamp(target_job.submit_time as i64, 0).expect("Could not determine")
    );
    println!(
        "Latest Start Time: {}",
        DateTime::from_timestamp(target_job.start_time as i64, 0).expect("Could not determine")
    );
    println!("Job status: {}", target_job.job_state);
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
    println!("Job directory: {}", target_job.current_working_directory);

    // Output file if it exists
    let output_file = Path::new(&target_job.standard_output);

    if output_file
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
    }
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
