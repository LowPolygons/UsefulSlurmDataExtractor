# SlurmHelper

A CLI for extracting useful information from SLURM on HPC systems

#### Note: This version is for any HPC system using version 26.05, which included some breaking changes to the json

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
