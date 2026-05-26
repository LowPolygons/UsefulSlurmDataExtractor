use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    containers::{
        SlurmMeta, SlurmSetInfiniteNumberContainer, useful_slurm_job_info::UsefulJobInfo,
    },
    systems::filter::ExtractsFilterableCategories,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SacctData {
    pub meta: SlurmMeta,
    pub warnings: Vec<HashMap<String, String>>,
    pub errors: Vec<HashMap<String, String>>,
    pub jobs: Vec<SacctJob>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SacctJob {
    pub account: String,
    pub comment: HashMap<String, String>,
    pub allocation_nodes: i64,
    pub array: SacctArray,
    pub association: HashMap<String, String>,
    pub block: String,
    pub cluster: String,
    pub container: String,
    pub derived_exit_code: SacctExitCode,
    pub time: SacctTime,
    pub exit_code: SacctExitCode,
    pub extra: String,
    pub failed_node: String,
    pub flags: Vec<String>,
    pub group: String,
    // ignoring 'het'
    pub job_id: u64,
    pub name: String,
    pub licenses: String,
    // ignoring mcs
    pub nodes: String,
    pub partition: String,
    pub hold: bool,
    pub priority: SlurmSetInfiniteNumberContainer,
    pub qos: String,
    pub required: SacctRequired,
    pub kill_request_user: String,
    // ignoring reservation
    pub script: String,
    pub state: HashMap<String, String>,
    // ignoring steps
    pub submit_line: String,
    pub tres: SacctTres,
    pub used_gres: String,
    pub user: String,
    // Ignoring wckey
    pub working_directory: String,
}

impl ExtractsFilterableCategories for SacctJob {
    fn get_directory(&self) -> String {
        self.working_directory.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_job_status(&self) -> String {
        self.exit_code.status.clone()
    }

    fn get_num_nodes(&self) -> u16 {
        0
    }

    fn get_account(&self) -> String {
        self.account.clone()
    }

    fn get_username(&self) -> String {
        self.user.clone()
    }
}

impl UsefulJobInfo for SacctJob {
    fn get_job_name(&self) -> &String {
        &self.name
    }

    fn get_job_id(&self) -> String {
        self.job_id.to_string()
    }

    fn get_user_name(&self) -> &String {
        &self.user
    }

    fn get_user_id(&self) -> String {
        String::from("N/A")
    }

    fn get_job_state(&self) -> &String {
        &self.exit_code.status
    }

    fn get_submit_time(&self) -> u64 {
        self.time.submission
    }

    fn get_start_time(&self) -> u64 {
        self.time.start
    }

    fn get_end_time(&self) -> u64 {
        self.time.end
    }

    fn get_directory(&self) -> &String {
        &self.working_directory
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SacctTres {
    pub allocated: Vec<SacctTresAllocReq>,
    pub requested: Vec<SacctTresAllocReq>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SacctTresAllocReq {
    #[serde(rename = "type")]
    pub key_is_type: String,
    pub name: String,
    pub id: i64,
    pub count: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SacctRequired {
    #[serde(rename = "CPUs")]
    pub cpus: i64,
    pub memory_per_cpu: SlurmSetInfiniteNumberContainer,
    pub memory_per_node: SlurmSetInfiniteNumberContainer,
    pub memory: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SacctArray {
    pub job_id: i64,
    // Ignoring 'limits' key
    pub task_id: SlurmSetInfiniteNumberContainer,
    pub task: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SacctExitCode {
    pub status: String,
    pub return_code: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SacctTime {
    pub elapsed: u64,
    pub eligible: u64,
    pub end: u64,
    pub start: u64,
    pub submission: u64,
    pub suspended: u64,
    pub system: HashMap<String, i64>,
    pub limit: SlurmSetInfiniteNumberContainer,
    pub total: HashMap<String, i64>,
    pub user: HashMap<String, i64>,
}
