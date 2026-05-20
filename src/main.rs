use std::{
    io::{self, IsTerminal, Read},
    os::unix::process::CommandExt,
    process::{Command, ExitCode},
};

use serde::de::DeserializeOwned;

mod cli;
mod commands;
mod containers;
mod systems;
mod utils;

use crate::{
    cli::{Cli, Commands},
    commands::{cancel_help, detail, list, list_directory, sinfo, system_capacity, tail_output},
    containers::slurm_data::{self, SlurmData},
};

use clap::Parser;

pub fn json_string_to_struct<T: DeserializeOwned>(stringy_json: String) -> Result<T, ()> {
    let structy_value = serde_json::from_str(&stringy_json).map_err(|_e| {
        println!("{}", _e.to_string());
        return ();
    })?;

    Ok(structy_value)
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Try extract piped input or run command manually
    let mut input = String::new();
    let structure: SlurmData;

    if io::stdin().is_terminal() {
        println!("User did not provide any input - Attempting to extract data manually");

        let squeue_output = if cli.all {
            // println!("Extracting data for all jobs");
            Command::new("squeue").arg("--json").output()
        } else {
            // println!("Extracting data for --me");
            Command::new("squeue").arg("--json").arg("--me").output()
        };

        match squeue_output {
            Ok(v) => {
                input = String::from_utf8_lossy(&v.stdout).to_string();
            }
            Err(_) => {
                println!("Failed to run squeue command internally, consider piping it in");
                return ExitCode::FAILURE;
            }
        }
    } else {
        let _ = io::stdin().read_to_string(&mut input).map_err(|_| {
            println!("Failed to read user input - did you run it like 'squeue --json | UsefulSlurmDataExtractor' ?");
            return ExitCode::FAILURE;
        });
    };

    structure = match json_string_to_struct(input) {
        Ok(val) => val,
        Err(_) => {
            println!("Failed to format input properly - consider piping the data in");
            return ExitCode::FAILURE;
        }
    };

    let success: Result<(), ()> = match &cli.command {
        Commands::Detail {
            job_id,
            filter,
            values,
        } => detail::command(&structure, job_id, filter, values),
        Commands::CancelHelp {
            directory,
            filter,
            values,
        } => cancel_help::command(directory, &structure, filter, values),
        Commands::ListDirectory { filter, values } => {
            list_directory::command(&structure, filter, values)
        }
        Commands::TailOutput {
            filter,
            values,
            num_lines,
        } => tail_output::command(&structure, filter, values, num_lines),
        Commands::SystemCapacity => system_capacity::command(&structure),
        Commands::List { filter, values } => list::command(&structure, filter, values),
        Commands::Sinfo => sinfo::command(),
    };

    match success {
        Ok(_) => {}
        Err(_) => {
            println!("Unsuccessful program execution");

            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
