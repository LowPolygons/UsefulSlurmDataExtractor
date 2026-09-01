# SlurmHelper

A CLI for extracting useful information from SLURM on HPC systems

## Warning

The main branch is guaranteed to work for Slurm 23. A change inbetween this version and version 26 caused many json changes that broke this version. If this is the case, checkout `slurm-26-05`

## Usage

Install Cargo on your machine

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Compile the program and add the `target/release` directory to your $PATH (recommended in .bashrc)

```sh
cargo build -r

export PATH=$PATH:/path/to/repo/target/release
```

```sh
squeue --json | SlurmHelper
```
Or
```sh
squeue --json --m | SlurmHelper
```

## TODO

Allow the option to pipe data in or to have the program run the commands internally
