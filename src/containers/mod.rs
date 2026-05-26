use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod sacct_data;
pub mod sinfo_data;
pub mod slurm_data;
pub mod useful_slurm_job_info;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlurmMeta {
    pub command: Vec<String>,
    pub plugins: HashMap<String, String>,
    #[serde(rename = "Slurm")]
    pub slurm: SlurmMetaSlurm,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlurmMetaSlurm {
    pub version: HashMap<String, i64>,
    pub release: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlurmSetInfiniteNumberContainer {
    pub set: bool,
    pub infinite: bool,
    pub number: f32,
}
