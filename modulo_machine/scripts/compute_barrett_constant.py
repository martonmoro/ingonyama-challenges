#!/usr/bin/env python3

def compute_barrett_constant(P):
    k = 2 * P.bit_length()
    R = (2**k) // P
    return R, k

def main():
    P = 0xE7EB417862865B8FF6FA5C28E93008D69368F209AD2757CC370682FE26BDC75D
    
    print("Computing Barrett constant R for modulo reduction")
    
    P_decimal = int(P)
    print(f"P (hex)     = {P:064x}")
    print(f"P (decimal) = {P_decimal}")
    print(f"P bit width = {P.bit_length()} bits")
    print()
    
    R, k = compute_barrett_constant(P)
    
    print(f"k (shift amount) = {k} bits")
    print(f"R = floor(2^{k} / P)")
    print()
    print(f"R (decimal) = {R}")
    print(f"R (hex)     = {R:065x}")
    print(f"R bit width = {R.bit_length()} bits")
    print()

if __name__ == "__main__":
    main()