use clap::{Parser, Subcommand};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum FilterOptions {
    Directory,
    Name,
    JobStatus,
    NumNodes,
    Account,
    Username,
    None,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    #[command(about = "Used to get a detailed list of info on a specific Job")]
    Detail {
        #[arg(long, required = false)]
        job_id: Option<u64>,

        #[arg(long, value_enum, default_value_t = FilterOptions::None)]
        filter: FilterOptions,

        #[arg(required = false, num_args = 1..)]
        values: Vec<String>,
    },

    #[command(about = "Used to aid in swiftly cancelling unwanted jobs")]
    CancelHelp {
        #[arg(long, default_value_t = String::from(""))]
        directory: String,

        #[arg(long, value_enum, default_value_t = FilterOptions::None)]
        filter: FilterOptions,

        #[arg(required = false, num_args = 1..)]
        values: Vec<String>,
    },

    #[command(about = "Used to see the list of files in the directory a job was from")]
    ListDirectory {
        #[arg(long, value_enum, default_value_t = FilterOptions::None)]
        filter: FilterOptions,

        #[arg(required = false, num_args = 1..)]
        values: Vec<String>,
    },

    #[command(about = "Used to view the tail (or full in the editor) of the output file for a job")]
    TailOutput {
        #[arg(long, value_enum, default_value_t = FilterOptions::None)]
        filter: FilterOptions,

        #[arg(required = false, num_args = 1..)]
        values: Vec<String>,

        #[arg(long)]
        editor: bool,
    },

    #[command(
        about = "Used to view the current capacity of the HPC system to get an idea of how busy it is"
    )]
    SystemCapacity,

    #[command(about = "See basic infomation about all current jobs in the list")]
    List,
}

#[derive(Parser)]
#[command(name = "slurmhelper")]
#[command(about = "SlurmHelper: A CLI For extracting useful information on a SLURM system")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
