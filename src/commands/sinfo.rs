use std::process::Command;

use crate::{containers::sinfo_data::SinfoData, json_string_to_struct};

pub fn command() -> Result<(), ()> {
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

    let structure: SinfoData = json_string_to_struct(input).map_err(|e| {
        println!("Failed to create sinfo structure from input");
        return ();
    })?;

    println!("Number of nodes: {}", structure.sinfo.len());
    Ok(())
}
