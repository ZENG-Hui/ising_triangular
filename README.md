# Ising Triangular
- Ising first neighbor on a triangular lattice
- Metropolis algorithm with spin flips and parallel tempering
- Coded in python and rust

## Rust module

    sweep(spins, temperature, n_flips=None)

- `spins` numpy 2D matrix of int32, modified by the function
- `temperature` for the flip probabilities
- `n_flips` amount of attempts to flip, by default it is the amount of spins

It returns the difference of energy between the new spin configuration and the original one.

## To run

- Install rust [here](https://www.rust-lang.org/en-US/install.html) or with `sudo apt-get install rustc cargo`
- Install python3 with `sudo apt-get install python3 python3-numpy python3-matplotlib`
- Install jupyther with `sudo apt-get install jupyter-notebook`
- Compile the rust code with the command `cargo build --release`
- Go on jupyter with `jupyter-notebook`
