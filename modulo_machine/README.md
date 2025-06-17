# Modulo Machine

Hardware implementation of modular reduction for computing `O = X mod P` where `X` is a 300-bit input and `P` is a 256-bit prime number.

**P = 104899928942039473597645237135751317405745389583683433800060134911610808289117**

## Overview

I implemented two different approaches to modular reduction in Verilog:

1. **Trivial Implementation** (`modulo_machine_trivial.v`) - Uses Verilog's built-in modulo operator
2. **Barrett Reduction** (`modulo_machine_barrett.v`) - Implements Barrett reduction algorithm for hardware synthesis

## Architecture

Both implementations follow the same interface:

- **Input Clock** (1 bit) - System clock
- **Input Reset** (1 bit) - Reset
- **Input X** (300 bits) - Number to be reduced
- **Output O** (256 bits) - Result of X mod P

## Implementation Details

### Trivial Solution

The trivial implementation uses Verilog's built-in modulo operator (`%`):

```verilog
wire [299:0] mod_result;
assign mod_result = X % P;
```

This approach works perfectly in simulation, but **cannot be synthesized** to actual hardware. The modulo operator with large bit widths requires complex division circuits that synthesis tools cannot automatically generate efficiently.

### Barrett Reduction

I chose Barrett reduction because it computes modular reduction without division, making it suitable for hardware implementation. The algorithm works by:

1. **Pre-computing a constant**: `R = floor(2^k / P)` where `k = 2 * bit_length(P)`
2. **Approximating the quotient**: `q = floor((X * R) / 2^k)`
3. **Computing remainder**: `remainder = X - q * P`
4. **Final corrections**: Since Barrett reduction gives an approximation, I need at most 2 subtractions to get the exact result

My Barrett implementation computes:

- `R = 0x11a94da6b4beeafe5851e6105114f39f593a0578c5d889b4e670b5c00558f50e9` (257 bits)
- Uses only additions, subtractions, multiplications, and bit shifts
- Requires 2 conditional subtractions for final correction

#### Implementation in Hardware

My Barrett implementation (`modulo_machine_barrett.v`) follows these steps:

```verilog
// Step 1: Multiply X by pre-computed Barrett constant R
assign x_times_r = X * R;

// Step 2: Extract quotient by right-shifting (dividing by 2^512)
assign q = x_times_r[556:512];  // Extract bits 556 down to 512 (45-bit quotient)

// Step 3: Compute q * P and subtract from X
assign q_times_p = q * P;
assign remainder = X - q_times_p[299:0];

// Step 4: Two conditional corrections (at most 2 subtractions needed)
assign reduced_once = (remainder >= {44'd0, P}) ? remainder[255:0] - P : remainder[255:0];
assign reduced_twice = (reduced_once >= P) ? reduced_once - P : reduced_once;
```

## Testing

**Build the comparison testbench**:

```bash
chmod +x build_compare.sh
./build_compare.sh
```

The script will:

- Compile both Verilog implementations with Verilator
- Build the C++ testbench with GMP support
- Run the comparison tests automatically

### Building Individual Tests

For testing just the trivial implementation:

```bash
verilator --cc --exe --build \
    src/modulo_machine_trivial.v \
    tb/testbench.cpp \
    -CFLAGS "$(pkg-config --cflags gmpxx)" \
    -LDFLAGS "$(pkg-config --libs gmpxx)"
./obj_dir/Vmodulo_machine_trivial
```

### Synthesis Comparison

To compare synthesis results between implementations:

```bash
cd scripts/
chmod +x compare_synthesis.sh
./compare_synthesis.sh
```

This script will:

- Attempt synthesis with Yosys (if available)
- Generate Verilator statistics for both implementations
- Keep in mind that in this form the trivial solution will not be properly synthetized

### Computing Barrett Constant

To recompute the Barrett constant for a different modulus:

```bash
cd scripts/
python3 compute_barrett_constant.py
```

## Expected Results

When running the comparison test, you should see:

```
Comparing Trivial vs Barrett Implementation
[PASS] Test 1: Both outputs match: 0x0
[PASS] Test 2: Both outputs match: 0x0
[PASS] Test 3: Both outputs match: 0x1
[PASS] Test 4: Both outputs match: 0x0
[PASS] Test 5: Both outputs match: 0x6a71baa307eee7783bb60c36cd26feae036276575b0ca0e2828b0b5b47d5527c

Running 10 random tests

[SUCCESS] All tests passed! Both implementations match.
```

## Notes

- The trivial implementation serves as a reference for verification but cannot be used in real hardware
- The Barrett implementation is the production-ready version suitable for hardware deployment
- I pre-computed the Barrett constant for the specific 256-bit prime P in the challenge using the python script
