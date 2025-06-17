# Ingonyama Challenge Solutions

My solutions to three Ingonyama technical challenges covering cryptography, hardware design, and zero-knowledge proofs.

## Solutions

### Broken Dlog Challenge

I needed to solve a discrete logarithm problem using leaked information that provided bounds on the search space. I chose Baby-Step Giant-Step for its efficiency and ease of implementation with this problem size.

I implemented both sequential and parallel versions in Rust. Initially I tried `num-bigint` but found `rug` performed much better for the large integer operations required.

Final results: `r = 996179739629170` and `x = 129741816436586536192511069033522723797805991085207391260653840826086090109`

The parallel implementation provides significant runtime improvements that justify the additional complexity.

Files:

- `broken_dlog/src/main.rs` - entry point with configurable algorithm selection
- `broken_dlog/src/rug_solution.rs` - optimized implementation using rug library
- `broken_dlog/src/bigint_solution.rs` - portable version with num-bigint
- `broken_dlog/solution.txt` - computed solution values

### A Modulo Machine

I designed a digital circuit to compute `O = X mod P` where X is 300 bits and P is a 256-bit prime.

I implemented Barrett reduction for hardware synthesis. The trivial approach using Verilog's `%` operator only works in simulation.

Barrett reduction precomputes a constant R that eliminates the expensive division operation. This makes it much more efficient for hardware implementation.

Files:

- `modulo_machine/src/modulo_compare.v` - top-level module comparing both approaches
- `modulo_machine/src/modulo_machine_barrett.v` - Barrett reduction implementation for synthesis
- `modulo_machine/src/modulo_machine_trivial.v` - reference implementation using built-in modulo
- `modulo_machine/tb/testbench.cpp` - C++ testbench with GMP for verification
- `modulo-machine/tb/testbench_compare.cpp` - equivalence testing between implementations

### Minimal Viable Prover

I built a commitment scheme optimized for efficiency using arkworks for the cryptographic primitives.

My approach only uses minimal amount of polynomial interpolation by working directly in Lagrange basis wherever it's possible.

I also implemented SRS caching between runs and parallelized the computationally expensive operations and used MSM to calculate the final EC point.

Files:

- `minimal_viable_prover/src/main.rs` - main workflow
- `minimal_viable_prover/src/prover.rs` - core polynomial commitment implementation
- `minimal_viable_prover/src/setup.rs` - trusted setup
