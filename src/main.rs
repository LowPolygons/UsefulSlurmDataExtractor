use std::{
    collections::HashMap,
    io::{self, IsTerminal, Read},
    process::ExitCode,
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
        list_directory::ListDirectory, sacct::Sacct, sinfo::Sinfo, system_capacity::SystemCapacity,
        tail_output::TailOutput,
    },
    containers::piped_input::{PipedInputHandler, StructOptions},
};

use clap::Parser;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let mut command_for_struct_args: HashMap<String, String> = HashMap::new();

    if cli.all {
        command_for_struct_args.insert("requires_all_in_queue".to_string(), "true".to_string());
    }

    #[rustfmt::skip]
    let command: Box<dyn CommandCall> = match cli.command {
        Commands::Detail {job_id,filter,values,} => 
            Box::new(Detail { filter, job_id, values }),
        Commands::CancelHelp { filter, values } => 
            Box::new(CancelHelp { filter, values }),
        Commands::ListDirectory { filter, values } => 
            Box::new(ListDirectory { filter, values }),
        Commands::TailOutput { filter,values,num_lines } => 
            Box::new(TailOutput { filter, values, num_lines }),
        Commands::List { filter, values } => 
            Box::new(List { filter, values }),
        Commands::Sinfo => Box::new(Sinfo {}),
        Commands::SystemCapacity => {
            command_for_struct_args.insert("requires_all_in_queue".to_string(), "true".to_string());

            Box::new(SystemCapacity {})
        }
        Commands::Sacct { user, days, filter, values } => { 
            command_for_struct_args.insert("user".to_string(), user.clone());

            if let Some(num_days) = days {
                command_for_struct_args.insert("days".to_string(), num_days.to_string());
            }
    
            Box::new(Sacct { days, filter, values })
        }
    };

    let piped_input_handler: Box<dyn PipedInputHandler> = command.get_piped_input_handler();

    let formated_struct: StructOptions = if io::stdin().is_terminal() {
        match piped_input_handler.try_run_command_to_get_struct(command_for_struct_args) {
            Ok(v) => v,
            Err(e) => {
                println!("Error trying to turn piped input into required structure - {e}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        let mut input = String::new();

        let _ = io::stdin().read_to_string(&mut input).map_err(|_| {
            println!("Failed to read user input - did you run it like '[slurm-command] --json | UsefulSlurmDataExtractor' ?");
            return ExitCode::FAILURE;
        });

        match piped_input_handler.try_make_piped_input_into_struct(input) {
            Ok(v) => v,
            Err(e) => {
                println!("Error trying to turn piped input into required structure - {e}");
                return ExitCode::FAILURE;
            }
        }
    };

    match command.command(&formated_struct) {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => {
            println!("Unsuccessful program execution");
            ExitCode::FAILURE
        }
    }
}
