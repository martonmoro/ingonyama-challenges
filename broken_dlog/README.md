# Broken Dlog

Discrete logarithm solver using Baby-Step Giant-Step algorithm.

## Problem

The challenge provides description for a Sigma protocol, where we know that Alice's pseudo-random number generator is broken and outputs only 50-bit numbers. Using this weakness the challenge asked the solver to extract a secret value from the protocol.

## What I Built

I implemented the BSGS algorithm for finding the logarithm in the dlog problem. I used two different backends (`bigint` and `rug`) and also paralellized my solution. These 4 option (`bigint`/`rug`, `sequential`/`parallel`) can be compared using the `bench.sh` script.

## Usage

```bash
# Run with backend, parallel flag, input file
cargo run --release rug true values.txt

# Or build first then run binary
cargo build --release
./target/release/broken_dlog rug true values.txt
```

## Performance Results

From my benchmarks:

- Rug parallel: ~77s
- Rug sequential: ~143s
- BigInt parallel: ~149s
- BigInt sequential: ~315s

These are single-run results for demonstration purposes.

## Input Format

```
p=<prime>
g=<generator>
h=<target>
a=<constraint_value>
c=<constraint_coeff>
t=<constraint_target>
```

## Algorithm Implementation

I used Baby-Step Giant-Step with m = 2^25 steps:

1. **Baby steps**: I compute g^i mod p for i = 0 to m-1, storing in HashMap for O(1) lookup
2. **Giant steps**: I compute a \* (g^(-m))^j mod p for j = 0 to m-1
   - Look for matches with baby step table
   - Use precomputed g^(-m) to avoid repeated modular inverse
3. **Collision found**: r = i + j\*m gives us g^r ≡ a (mod p)
4. **Recover x**: Use additional constraints (c, t values) to solve for final x

The parallel version chunks the giant step phase across threads with 1M iterations per chunk. Each thread maintains its own starting value and builds the powers iteratively trying to balance out the benefits of parallel execution with the downside of having to do more modpows instead of iterative modmuls to calculate the powers.

## Constraint Solving

To recover x from the found value r, I use the constraint equation:

```
t ≡ r + cx (mod φ(p))
```

In the case of this challenge c and φ(p) are not coprimes (both are even), so the code handles that case with a fallback solution that check if integer division is possible and if yes then uses it instead of modular inverse calculation.

1. **Primary method**: I attempt to find the modular inverse of c modulo φ(p)

   - If c^(-1) exists: x = (t - r) \* c^(-1) (mod φ(p))

2. **Fallback method**: When c and φ(p) are not coprime (no modular inverse exists)
   - I check if (t - r) is divisible by c as integers
   - If divisible: x = (t - r) / c (using integer division)
   - This handles cases where the constraint equation has an integer solution even without a modular inverse

This dual approach ensures my solver can handle a wider range of challenge parameters.

## Modular Inverse Calculation

I compute the modular inverse of c modulo φ(p) using the Extended Euclidean Algorithm:

Given integers c and φ(p), I find coefficients x and y such that gcd(c, φ(p)) = cx + φ(p)y. If gcd(c, φ(p)) = 1, then c^(-1) ≡ x (mod φ(p)).

The main steps are:

- Initialize: old_remainder = c, remainder = φ(p), old_a_coeff = 1, curr_a_coeff = 0
- While remainder ≠ 0: compute quotient and update coefficients
- Return old_a_coeff (adjusted for positive result) as the modular inverse

## Alternative Approaches

I considered Pollard's Kangaroo algorithm as an alternative to BSGS. It would use O(1) memory instead of O(√n), which could be beneficial for larger problems. The algorithm also parallelizes well with multiple kangaroos from different starting points. Given the bounded search space from the leaked information, kangaroo would be well-suited to this problem. I ended up choosing BSGS for ease of implementation.
