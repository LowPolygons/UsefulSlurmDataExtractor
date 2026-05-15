# SlurmHelper

A CLI for extracting useful information from SLURM on HPC systems

## Usage

Compile the program and add the `target/release` directory to your $PATH

```sh
squeue --json | SlurmHelper
```
Or
```sh
squeue --json --m | SlurmHelper
```

## TODO

Various features I wish to implement

### Detailed Mode

Potential usage:
```sh
squeue --json --m | SlurmHelper --detailed
```

Opens an interactive menu which lists all the current jobs from the piped json

The list can be filtered out perhaps with an optional command. EG:

Only lists pending jobs
```sh
... | SlurmHelper --detailed --filter status pending
```

Only lists jobs which are in `/x/y/z`
```sh
... | SlurmHelper --detailed --filter directory x/y/z 
```

Once a job is selected, it opens a menu which lists all the useful information on it

Alternatively, the user can pass a job id to skip the interactive menu
```sh
... | SlurmHelper --detailed --job-id 123456789 
```

### Cancel Helper

Potential Usage:
```sh
squeue --json -m | SlurmHelper --cancel-help
```

Again, opens an interactive menu which lists all the current jobs from the piped json

The list can also be filtered out as with detailed-mode

Only lists pending jobs
```sh
... | SlurmHelper --cancel-help --filter status pending
```

Only lists jobs which are in `/x/y/z`
```sh
... | SlurmHelper --cancel-help --filter directory x/y/z 
```

The user can then select the jobs they wish to be cancelled, with an optional 'All' shorthand

Once finished, it creates a `slurm_helper_cancel_script.sh` which will cancel them all

Alternatively, the user has an option to bypass the interactive menu to cancel all jobs in a passed directory
```sh
... | SlurmHelper --cancel-help --directory x/y/z 
```
