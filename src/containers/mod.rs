use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod piped_input;
pub mod sacct_data;
pub mod sacct_handler;
pub mod sinfo_data;
pub mod sinfo_handler;
pub mod slurm_data;
pub mod slurm_handler;
pub mod useful_slurm_job_info;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlurmMeta {
    pub command: Vec<String>,
    pub plugin: HashMap<String, String>,
    pub slurm: SlurmMetaSlurm,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlurmMetaSlurm {
    pub version: HashMap<String, String>,
    pub release: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlurmSetInfiniteNumberContainer {
    pub set: bool,
    pub infinite: bool,
    pub number: f32,
}
