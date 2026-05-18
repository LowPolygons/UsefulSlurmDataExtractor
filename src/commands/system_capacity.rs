use crate::containers::slurm_data::SlurmData;

pub fn command(structure: &SlurmData) -> Result<(), ()> {
    let running_pending: Vec<usize> = structure.jobs.iter().fold(vec![0, 0], |mut vec, job| {
        match job.job_state.as_str() {
            "PENDING" => vec[1] = vec[1] + 1,
            "RUNNING" => vec[0] = vec[0] + 1,
            _ => {}
        }
        // if !job.dependency.is_empty() {
        //     println!(
        //         "{} and {} and {}!",
        //         job.job_id, job.dependency, job.state_reason
        //     );
        // }
        return vec;
    });

    println!("There are {} jobs in total", structure.jobs.len());
    println!(
        "In total, {} are 'RUNNING' and {} are 'PENDING', and {} other",
        running_pending[0],
        running_pending[1],
        structure.jobs.len() - running_pending[0] - running_pending[1]
    );
    Ok(())
}
