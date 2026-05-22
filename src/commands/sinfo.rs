use std::process::Command;

use crate::{
    commands::command::CommandCall,
    containers::{sinfo_data::SinfoData, slurm_data::SlurmData},
    json_string_to_struct,
};

pub struct Sinfo {}

impl CommandCall for Sinfo {
    fn command(&self, _: &SlurmData) -> Result<(), ()> {
        let sinfo_output = Command::new("sinfo").arg("--json").output();
        let input: String;

        match sinfo_output {
            Ok(v) => {
                input = String::from_utf8_lossy(&v.stdout).to_string();
            }
            Err(_) => {
                println!("Failed to run sinfo command");
                return Err(());
            }
        }

        let structure: SinfoData = json_string_to_struct(input).map_err(|_e| {
            println!("Failed to create sinfo structure from input");
            return ();
        })?;

        println!("Number of nodes: {}", structure.sinfo.len());

        structure.sinfo.iter().for_each(|sinfo| {
            println!(
                "- {} has {} cpus. {} are idle, {} are allocated and {} are other",
                sinfo.partition.name,
                sinfo.cpus.total,
                sinfo.cpus.idle,
                sinfo.cpus.allocated,
                sinfo.cpus.other
            );
        });
        Ok(())
    }
}
