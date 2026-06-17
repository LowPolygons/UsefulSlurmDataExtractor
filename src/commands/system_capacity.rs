use crate::{
    commands::command::CommandCall,
    containers::{
        piped_input::{PipedInputHandler, StructOptions},
        slurm_data::SlurmData,
        slurm_handler::SlurmHandler,
    },
};

pub struct SystemCapacity {}

impl CommandCall for SystemCapacity {
    fn command(&self, slurm_data: &StructOptions) -> Result<(), ()> {
        let structure: &SlurmData = match slurm_data {
            StructOptions::Slurm(slurm_data) => slurm_data,
            StructOptions::Sacct(sacct_data) => return Err(()),
            StructOptions::Sinfo(sinfo_data) => return Err(()),
        };

        let running_pending_and_node_counts: Vec<usize> =
            structure
                .jobs
                .iter()
                .fold(vec![0, 0, 0, 0], |mut vec, job| {
                    match job.job_state.as_str() {
                        "RUNNING" => {
                            vec[0] = vec[0] + 1;
                            vec[2] = vec[2] + job.node_count.number as usize;
                        }
                        "PENDING" => {
                            vec[1] = vec[1] + 1;
                            vec[3] = vec[3] + job.node_count.number as usize;
                        }
                        _ => {}
                    }
                    return vec;
                });

        println!("There are {} jobs in total", structure.jobs.len());
        println!(
            "In total, {} are 'RUNNING' and {} are 'PENDING', and {} other",
            running_pending_and_node_counts[0],
            running_pending_and_node_counts[1],
            structure.jobs.len()
                - running_pending_and_node_counts[0]
                - running_pending_and_node_counts[1]
        );
        println!(
            "The running jobs are using {} nodes, and the pending jobs will use {} nodes",
            running_pending_and_node_counts[2], running_pending_and_node_counts[3]
        );
        Ok(())
    }

    fn get_piped_input_handler(&self) -> Box<dyn PipedInputHandler> {
        return Box::new(SlurmHandler::new());
    }
}
