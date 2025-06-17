# Minimal Viable Prover

Minimal Viable Prover implemented using arkworks

## Problem

The challenge provides description for a commitment protocol and ask the challenge solver to improve on its inefficiencies.

## My Approach

I built an optimized prover implementation trying to optimize both the protocol and the way the computation is done. The core change is avoiding polynomial interpolation as much as possible by working directly in Lagrange basis and also caching the SRS for consequent runs of the protocol.

## Performance Optimizations

I did the following optimizations.

- **Lagrange basis SRS**: Generate SRS directly as g^{L_i(τ)} using FFT.
- **Evaluation domain operations**: All polynomial operations work directly in evaluation form
- **Direct blinding generation**: Generate blinding polynomial directly in evaluation form using parallel random generation
- **Hadamard product blinding**: Apply blinding via element-wise multiplication in evaluation space
- **Single MSM commitment**: Use multi-scalar multiplication directly on evaluation domain products
- **Persistent SRS caching**: Store and reload Lagrange SRS using arkworks serialization (unchecked for performance reasons)
- **Parallel scalar operations**: Multi-threaded blinding generation and Hadamard products

## Protocol Comparison

**Original Protocol:**

1. Generate random τ ∈ F, and compute {1, τ, τ^2, ..., τ^(2n-1)}
2. Generate random group element G ∈ G_1 of BLS12-381 and compute SRS in monomial basis [G]\_SRS = {G, τ·G, τ^2·G, ..., τ^(2n-1)·G} = {G_0, G_1, ..., G_2n-1}
3. Convert it into Lagrange basis
4. Generate random polynomial with c_i ∈ F_r in Lagrange basis: c_2n^eval = {c_0, c_1, ..., c_2n-1}
5. **Witness**: Let x_i ∈ F_r for i = 0, 1, ..., n-1
6. Compute f_i = Hash(x_i); f_i ∈ F_r
7. Convert vector of f_i into vector of length 2n using FFT: f_2n^eval = FFT_2n[f_0, f_1, ..., f_n-1||0_n]
8. Compute the commitment: G_comm = (c_2n^eval ∘ f_2n^eval)^T · [G]\_SRS^Lag

Where ∘ refers to Hadamard product, and · is Multiscalar multiplication.

**My Protocol:**

Looking at your protocol and comparing it to the detailed original protocol steps, your "My Protocol" section is actually well-structured and captures the key optimizations. However, to match the level of detail in the original protocol, you could add a bit more specificity:

**My Protocol:**

1. Generate Lagrange SRS directly: g^{L_i(τ)} via FFT of powers of τ (skip monomial basis entirely)
2. Convert witness to evaluation form using single FFT: f_2n^eval = FFT_2n[f_0, f_1, ..., f_n-1||0_n]
3. Generate blinding polynomial directly in evaluation domain: c_2n^eval = {c_0, c_1, ..., c_2n-1} (parallel random generation)
4. Apply blinding via Hadamard product: product = c_2n^eval ∘ f_2n^eval (element-wise multiplication)
5. Single MSM for final commitment: G_comm = product^T · [G]\_SRS^Lag

My Lagrange approach eliminates the expensive many interpolation steps and keeps all operations in the evaluation domain.

## Usage

```bash
# Run without caching
cargo run --release

# Use cached SRS
cargo run --release true
```

## Technical Implementation

The prover works by:

1. **Setup Phase**: Generate Lagrange basis SRS using FFT operations
2. **Witness Processing**: Work directly with evaluation form of witness polynomial
3. **Blinding**: Apply random blinding factors in evaluation space using Hadamard products
4. **Commitment**: Single multi-scalar multiplication for final polynomial commitment

## Caching Strategy

I implemented persistent SRS caching to avoid recomputation:

- SRS files are stored with unique names based on domain size
- I used the unchecked version of the serialization functions for speed. Using the standard functions that run all the necessary validations make the caching not worth it since it is faster to regenerate the values in case we don't want to use the same SRS.
- Significant speedup for repeated proving operations
