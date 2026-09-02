use crate::{
    commands::command::CommandCall,
    containers::{
        piped_input::{PipedInputHandler, StructOptions},
        sinfo_data::CpuAndPartitionInfo,
        sinfo_data::SinfoData,
        sinfo_handler::SinfoHandler,
    },
};

pub struct Sinfo {}

impl CommandCall for Sinfo {
    fn command(&self, slurm_data: &StructOptions) -> Result<(), ()> {
        let structure: &SinfoData = match slurm_data {
            StructOptions::Slurm(_) => return Err(()),
            StructOptions::Sacct(_) => return Err(()),
            StructOptions::Sinfo(sinfo_data) => sinfo_data,
        };

        println!("Number of nodes: {}", structure.sinfo.len());

        structure.get_sinfo_data().iter().for_each(|sinfo| {
            println!(
                "- {} has {} cpus. {} are idle, {} are allocated and {} are other",
                sinfo.get_partition_name(),
                sinfo.get_total_cpus(),
                sinfo.get_idle_cpus(),
                sinfo.get_allocated_cpus(),
                sinfo.get_misc_cpus()
            );
        });
        Ok(())
    }

    fn get_piped_input_handler(&self) -> Box<dyn PipedInputHandler> {
        return Box::new(SinfoHandler::new());
    }
}
