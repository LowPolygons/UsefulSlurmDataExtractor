use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::containers::{SlurmMeta, SlurmSetInfiniteNumberContainer};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SinfoData {
    pub meta: SlurmMeta,
    pub warnings: Vec<HashMap<String, String>>,
    pub errors: Vec<HashMap<String, String>>,
    pub sinfo: Vec<SinfoValue>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SinfoValue {
    pub port: i64,
    pub node: HashMap<String, Vec<String>>,
    pub nodes: NodesInfo,
    pub cpus: CpuInfo,
    pub sockets: HashMap<String, i64>,
    pub cores: HashMap<String, i64>,
    pub threads: HashMap<String, i64>,
    pub disk: HashMap<String, i64>,
    pub memory: MemoryStruct,
    pub weight: HashMap<String, i64>,
    pub features: HashMap<String, String>,
    pub gres: HashMap<String, String>,
    pub cluster: String,
    pub comment: String,
    pub extra: String,
    pub reason: HashMap<String, String>,
    pub reservation: String,
    pub partition: Partition,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Partition {
    pub nodes: PartitionNodes,
    pub accounts: HashMap<String, String>,
    pub groups: HashMap<String, String>,
    pub qos: HashMap<String, String>,
    pub alternate: String,
    pub tres: HashMap<String, String>,
    pub cluster: String,
    pub cpus: HashMap<String, i64>,
    pub defaults: PartitionDefaults,
    pub grace_time: i64,
    pub maximums: PartitionMaximums,
    pub minimums: HashMap<String, i64>,
    pub name: String,
    pub node_sets: String,
    pub priority: HashMap<String, i64>,
    pub timeouts: HashMap<String, SlurmSetInfiniteNumberContainer>,
    pub suspend_time: SlurmSetInfiniteNumberContainer,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PartitionMaximums {
    pub cpus_per_node: SlurmSetInfiniteNumberContainer,
    pub cpus_per_socket: SlurmSetInfiniteNumberContainer,
    pub memory_per_cpu: i64,
    pub nodes: SlurmSetInfiniteNumberContainer,
    pub shares: i64,
    pub time: SlurmSetInfiniteNumberContainer,
    pub over_time_limit: SlurmSetInfiniteNumberContainer,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PartitionDefaults {
    pub memory_per_cpu: i64,
    pub time: SlurmSetInfiniteNumberContainer,
    pub job: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PartitionNodes {
    pub allowed_allocation: String,
    pub configured: String,
    pub total: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MemoryStruct {
    pub minimum: i64,
    pub maximum: i64,
    pub free: HashMap<String, SlurmSetInfiniteNumberContainer>,
    pub allocated: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NodesInfo {
    pub allocated: i64,
    pub idle: i64,
    pub other: i64,
    pub total: i64,
    pub hostnames: Vec<String>,
    pub addresses: Vec<String>,
    pub nodes: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CpuInfo {
    pub allocated: i64,
    pub idle: i64,
    pub other: i64,
    pub total: i64,
    pub minimum: i64,
    pub maximum: i64,
    pub load: HashMap<String, i64>,
    pub per_node: HashMap<String, SlurmSetInfiniteNumberContainer>,
}
