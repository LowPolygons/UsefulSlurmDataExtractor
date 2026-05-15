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

Only lists jobs which are in `x/y/z`
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

Only lists jobs which are in `x/y/z`
```sh
... | SlurmHelper --cancel-help --filter directory x/y/z 
```

The user can then select the jobs they wish to be cancelled, with an optional 'All' shorthand

Once finished, it creates a `slurm_helper_cancel_script.sh` which will cancel them all

Alternatively, the user has an option to bypass the interactive menu to cancel all jobs in a passed directory
```sh
... | SlurmHelper --cancel-help --directory x/y/z 
```

#### `--filter` Options
- Directory
- Name
- Job Status
- Num Nodes
- Department/Account
- Username

### List Working Directory

Potential Usage:
```sh
... | SlurmHelper --list-directory
```

Picking the file can be similar to detailed mode

It then lists the files in that directory

### View Output File Tail

Potential Usage:
```sh
... | SlurmHelper --tail-output [num lines]
```

Picking the file can be similar to detailed mode

It then prints the last [num lines] lines in the output file

Alternatively, a flag can be passed which will just open the output file in your editor
```sh
... | SlurmHelper --tail-output --editor
```

### System Capacity

Potential Usage
```sh
... | SlurmHerlp --system-capacity
```

Takes the list of jobs and prints useful information relating to how 'full' the queue is

For example, it may print that thwere are 500 jobs, 50 are running and 450 are PENDING
