# SlurmHelper

A CLI for extracting useful information from SLURM on HPC systems

#### Node: This version was built around slurm-23. If it fails to work for you and your version is higher, consider `slurm-26-05`

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
