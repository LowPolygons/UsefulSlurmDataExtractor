# UsefulSlurmDataExtractor

Right now, this will extract some info about each slurm command and tell you the formatted start and expected run date of it

## Usage

Compile the program and add the `target/release` directory to your $PATH

```sh
squeue --json | UsefulSlurmDataExtractor
```
Or
```sh
squeue --json --m | UsefulSlurmDataExtractor
```

