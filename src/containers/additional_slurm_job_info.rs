pub trait AdditionalJobInfo {
    fn get_time_limit(&self) -> u64;
    fn get_standard_output(&self) -> &String;
    fn get_standard_error(&self) -> &String;
    fn get_node_count(&self) -> u64;
    fn get_tasks_per_node(&self) -> u64;
}
