# Commands

When using SlurmHelper, there are two ways of providing input:

### Piping
```sh
squeue --json | SlurmHelper ..
```

Or letting the program run the 'slurm' command for you
```sg
SlurmHelper ...

(or)

SlurmHelper --all ...
```

Piping was implemented to to make development easier, and to allow examination of slurm results on other machines

For example:
```sh
squeue --json > output.txt

(transfer off HPC system)

cat output.txt | SlurmHelper ...
```

Every command will work regardless of your input

## List
```sh
[... | ]  SlurmHelper list 
        [--filter [filter_name] [values]]
```

This command, for each job found in the list, will print some information about the job

Example
```sh
>>> squeue --me --json | SlurmHelper list --filter job-status PENDING

==========================
Job Name & ID: [name], [id] 
User Name and ID: [username], [user_id]
Job status: PENDING
--------------------------
Submit Time: 20XX-XX-XX XX:XX:XX UTC
Start Time: 20XX-XX-XX XX:XX:XX UTC
Job directory: /dir/sbatch/was/run/from
==========================
...
==========================
Listed info for [n] jobs
==========================
```

## Detail
```sh
[... | ] SlurmHelper detail
        [--filter [filter_name] [values]] [--job-id [ID]]
```

This command will allow you to select a job from a list - or it will pick the job with the ID you provided - and then show more information on that job

Example
```sh
>>> squeue --me --json | SlurmHelper detail --filter job-status RUNNING

==========================
Job Name & ID: [name], [id] 
User Name and ID: [username], [user_id]
Job status: PENDING
--------------------------
Submit Time: 20XX-XX-XX XX:XX:XX UTC
Start Time: 20XX-XX-XX XX:XX:XX UTC
Running Time: XX:XX:XX
Job directory: /dir/sbatch/was/run/from
--------------------------
Files in working directory:
        ...
.. Some files Hidden ..
--------------------------
Job max length: XX.xx hours
Number of nodes: X
Number of tasks per node: X
--------------------------
Output File: /path/to/output/file
[output file line 1]
[output file line 2]
[output file line 3]
    ... Supressed (X) Lines
[output file line n-2]
[output file line n-1]
[output file line 0]
--------------------------
```

When on a job, you will also have the option to cancel it from a menu.

If the job has a separate error file, it will also print the contents of the error file (if there is one)

## List Directory
```sh
[... | ] SlurmHelper list-directory 
        [--filter [filter_name] [values]] 
```

This will allow you to select a job from a list and it will then print the full working directory from where the job is based. 

It will not hide any files, but it is NOT recursive

Example:
```sh
>>> SlurmHelper list-directory
..File.. "/path/to/dir/.."
..File.. "/path/to/dir/.."
..File.. "/path/to/dir/.."
..File.. "/path/to/dir/.."
..File.. "/path/to/dir/.."
..File.. "/path/to/dir/.."
..File.. "/path/to/dir/.."
..File.. "/path/to/dir/.."
..File.. "/path/to/dir/.."
..File.. "/path/to/dir/.."
..File.. "/path/to/dir/.."
..File.. "/path/to/dir/.."
```

## Tail Output
```sh
[... | ] SlurmHelper tail-output
        [--filter [filter_name] [values]] [--num-lines [n]]
```

This will allow you to select a job and then it will print the last --num-lines (or 30 default) lines of the output file

This will automatically remove jobs which are pending from the list

Example:
```sh
>>> SlurmHelper --all tail-output

------------------------
The last 30 lines of the output file: 
...
.. [output file line n-2]
.. [output file line n-1]
.. [output file line 0]
```

## Cancel Help
```sh
[... | ] SlurmHelper cancel-help
        [--filter [filter_name] [values]] 
```

This will allow you select any number of jobs, and it will create a script for you to run which will cancel all the jobs (and remove itself)

It will NOT automatically execute the script

Example:
```sh
>>> SlurmHelper cancel-help

[select the jobs]

Wrote the file 'slurm_helper_cancel_script.sh' to your current directory.

>>> cat slurm_helper_cancel_script.sh

rm /full/path/to/script/slurm_helper_cancel_script.sh
scancel ..
scancel ..
scancel ..
scancel ..
```

## System Capacity
```sh
[..] SlurmHelper system-capacity
```

This command will ignore any provided user input, and will default to running as if it was called like:
```sh
Slurmhelper --all system-capacity 
```

This will return a small report on the number of jobs running and pending, and what resources these are using/will use

Example:
```sh
>>> Slurmhelper system-capacity

There are 718 jobs in total
In total, 339 are 'RUNNING' and 377 are 'PENDING', and 2 other
The running jobs are using 1049 nodes, and the pending jobs will use 1600 nodes
```

## Sacct
```sh
SlurmHelper sacct
        --user [username] 
        [--filter [filter] [values]]
        [--days [num days back]]
```

This will list information extracted from the 'sacct' command, showing information about previous jobs.

If the user does not provide the --days flag, it defaults to 100 days

Example:
```sh
>>> SlurmHelper sacct --user [username] --days N
============================
Job Name & ID: [NAME], [ID] 
User Name and ID: [username], N/A
Job status: COMPLETED
--------------------------
Submit Time: 20XX-XX-XX XX:XX:XX UTC
Start Time: 20XX-XX-XX XX:XX:XX UTC
Running Time: XX:XX:XX
Job directory: /dir/sbatch/was/run/from
============================
Listed info for 1 jobs
============================
```
