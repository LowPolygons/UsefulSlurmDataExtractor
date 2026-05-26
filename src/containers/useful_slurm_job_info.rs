pub trait UsefulJobInfo {
    //
    fn get_job_name(&self) -> &String;
    fn get_job_id(&self) -> String;
    //
    fn get_user_name(&self) -> &String;
    fn get_user_id(&self) -> String;
    //
    fn get_job_state(&self) -> &String;
    //
    fn get_submit_time(&self) -> u64;
    fn get_start_time(&self) -> u64;
    fn get_end_time(&self) -> u64;
    //
    fn get_directory(&self) -> &String;
}
