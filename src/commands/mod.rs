use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::containers::slurm_data::SlurmJob;

pub mod cancel_help;
pub mod detail;
pub mod list;
pub mod list_directory;
pub mod system_capacity;
pub mod tail_output;

pub fn line_vec_from_file(file_name: &str) -> Result<Vec<String>, String> {
    let file: File = File::open(file_name).map_err(|e| e.to_string())?;

    return Ok(io::BufReader::new(file).lines().into_iter().try_fold(
        Vec::<String>::new(),
        |mut vec, line| -> Result<Vec<String>, String> {
            vec.push(String::from(line.map_err(|e| e.to_string())?));

            return Ok(vec);
        },
    )?);
}
