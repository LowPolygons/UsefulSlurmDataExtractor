use crate::containers::slurm_data::SlurmData;

pub trait CommandCall {
    fn command(&self, structure: &SlurmData) -> Result<(), ()>;
}
