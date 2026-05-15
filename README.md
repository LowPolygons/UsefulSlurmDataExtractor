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

#### 'Detailed Mode'

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

