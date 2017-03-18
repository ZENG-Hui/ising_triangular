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
