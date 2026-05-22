use std::{
    io::{self, IsTerminal, Read},
    process::{Command, ExitCode},
};

mod cli;
mod commands;
mod containers;
mod systems;
mod utils;

use crate::{
    cli::{Cli, Commands},
    commands::{
        cancel_help::CancelHelp, command::CommandCall, detail::Detail, list::List,
        list_directory::ListDirectory, sinfo::Sinfo, system_capacity::SystemCapacity,
        tail_output::TailOutput,
    },
    containers::slurm_data::SlurmData,
    utils::json_string_to_struct::json_string_to_struct,
};

use clap::Parser;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let mut requires_all_in_queue: bool = false;
    let command: Box<dyn CommandCall> = match cli.command {
        Commands::Detail {
            job_id,
            filter,
            values,
        } => Box::new(Detail {
            filter,
            job_id,
            values,
        }),
        Commands::CancelHelp {
            directory,
            filter,
            values,
        } => Box::new(CancelHelp {
            directory,
            filter,
            values,
        }),
        Commands::ListDirectory { filter, values } => Box::new(ListDirectory { filter, values }),
        Commands::TailOutput {
            filter,
            values,
            num_lines,
        } => Box::new(TailOutput {
            filter,
            values,
            num_lines,
        }),
        Commands::SystemCapacity => {
            requires_all_in_queue = true;

            println!("Attempting to extract all data from slurm");

            Box::new(SystemCapacity {})
        }
        Commands::List { filter, values } => Box::new(List { filter, values }),
        Commands::Sinfo => Box::new(Sinfo {}),
    };

    let structure: SlurmData = match get_structure(cli.all, requires_all_in_queue) {
        Ok(val) => val,
        Err(_) => {
            println!("Failed to create data structure");

            return ExitCode::FAILURE;
        }
    };

    match command.command(&structure) {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => {
            println!("Unsuccessful program execution");
            ExitCode::FAILURE
        }
    }
}

fn get_structure(cli_all: bool, requires_all_in_queue: bool) -> Result<SlurmData, ()> {
    let mut input = String::new();
    // The SystemCapacity command is useless if the user tries to use
    if io::stdin().is_terminal() || requires_all_in_queue {
        let squeue_output = if cli_all || requires_all_in_queue {
            Command::new("squeue").arg("--json").output()
        } else {
            Command::new("squeue").arg("--json").arg("--me").output()
        };

        match squeue_output {
            Ok(v) => {
                input = String::from_utf8_lossy(&v.stdout).to_string();
            }
            Err(_) => {
                println!("Failed to run squeue command internally, consider piping it in");
                return Err(());
            }
        }
    } else {
        let _ = io::stdin().read_to_string(&mut input).map_err(|_| {
            println!("Failed to read user input - did you run it like 'squeue --json | UsefulSlurmDataExtractor' ?");
            return ExitCode::FAILURE;
        });
    };

    let structure = match json_string_to_struct(input) {
        Ok(val) => val,
        Err(_) => {
            println!("Failed to format input properly - consider piping the data in");
            return Err(());
        }
    };

    Ok(structure)
}
