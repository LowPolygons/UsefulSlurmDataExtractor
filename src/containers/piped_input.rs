use std::collections::HashMap;


use crate::containers::{sacct_data::SacctData, sinfo_data::SinfoData, slurm_data::SlurmData};

pub enum StructOptions {
    Slurm(SlurmData),
    Sacct(SacctData),
    Sinfo(SinfoData),
}

pub trait PipedInputHandler {
    fn try_make_piped_input_into_struct(&self, input: String) -> Result<StructOptions, String>;

    fn try_run_command_to_get_struct(
        &self,
        args: HashMap<String, String>,
    ) -> Result<StructOptions, String>;
}
