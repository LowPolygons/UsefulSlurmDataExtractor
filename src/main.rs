use std::{
    io::{self, IsTerminal, Read},
    process::ExitCode,
};

use serde::de::DeserializeOwned;

mod cli;
mod commands;
mod containers;

use crate::{
    cli::{Cli, Commands},
    commands::list,
    containers::slurm_data,
};

use clap::Parser;

pub fn json_string_to_struct<T: DeserializeOwned>(stringy_json: String) -> Result<T, ()> {
    let structy_value = serde_json::from_str(&stringy_json).map_err(|_| {
        return ();
    })?;

    Ok(structy_value)
}

fn main() -> ExitCode {
    // Extracts the information from the piped input
    let mut input = String::new();

    if io::stdin().is_terminal() {
        println!(
            "User did not provide any input - did you run it like 'squeue --json | UsefulSlurmDataExtractor' ?"
        );
        return ExitCode::FAILURE;
    }

    let _ = io::stdin().read_to_string(&mut input).map_err(|_| {
        println!("Failed to read user input - did you run it like 'squeue --json | UsefulSlurmDataExtractor' ?");
        return ExitCode::FAILURE;
    });

    let structure: slurm_data::SlurmData = match json_string_to_struct(input) {
        Ok(val) => val,
        Err(_) => {
            println!(
                "Failed to format input properly - did you run it like 'squeue --json | UsefulSlurmDataExtractor' ?"
            );
            return ExitCode::FAILURE;
        }
    };

    let cli = Cli::parse();

    let success: Result<(), ()> = match &cli.command {
        Commands::Detail { filter, job_id } => {
            unimplemented!();
        }
        Commands::CancelHelp { filter, directory } => {
            unimplemented!()
        }
        Commands::ListDirectory { filter } => {
            unimplemented!()
        }
        Commands::TailOutput { filter, editor } => {
            unimplemented!()
        }
        Commands::SystemCapacity => unimplemented!(),
        Commands::List => list::command(&structure),
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
