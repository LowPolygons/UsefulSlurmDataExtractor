pub trait CpuJobInfo {
    fn get_number_of_cpus(&self) -> u64;
    fn get_memory_per_cpu(&self) -> u64;
}
