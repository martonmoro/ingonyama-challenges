use std::{collections::HashMap, fs, time::Instant};

use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{One, Zero};

use rayon::prelude::*;

struct ProtocolValues {
    p: BigUint,
    g: BigUint,
    h: BigUint,
    a: BigUint,
    c: BigUint,
    t: BigUint,
}

fn parse_input(filename: &str) -> Result<ProtocolValues, Box<dyn std::error::Error>> {
    let input = fs::read_to_string(filename)?;

    let mut values = HashMap::new();
    for line in input.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let value_big_uint = BigUint::parse_bytes(value.trim().as_bytes(), 10)
                .ok_or(format!("Failed to parse value for key: {}", key))?;
            values.insert(key.trim(), value_big_uint);
        }
    }

    Ok(ProtocolValues {
        p: values["p"].clone(),
        g: values["g"].clone(),
        h: values["h"].clone(),
        a: values["a"].clone(),
        c: values["c"].clone(),
        t: values["t"].clone(),
    })
}

fn find_r_bsgs(values: &ProtocolValues, use_parallel: bool) -> Option<u64> {
    let mut baby_steps: HashMap<BigUint, u32> = HashMap::new(); // Changed value type to u32

    // r = i + jm
    // i, j ∈ [0, m-1]
    // r is in range [0, 2^50-1]
    // so m = sqrt(2^50) = 2^25
    let m = 1u32 << 25;

    // baby-steps
    let mut g_to_i = BigUint::one();
    for i in 0..m {
        baby_steps.insert(g_to_i.clone(), i);
        g_to_i = (&g_to_i * &values.g) % &values.p;
    }

    // giant-steps
    let g_to_m = values.g.modpow(&BigUint::from(m), &values.p);
    // because of Fermat's Little Theorem
    // g_to_m^(-1) ≡ g_to_m^(p-2) (mod p)
    let p_minus_1 = &values.p - BigUint::from(1u64);
    let g_to_minus_m = g_to_m.modpow(&(&p_minus_1 - BigUint::from(1u64)), &values.p);

    if !use_parallel {
        // sequential solution
        let mut g_to_m_to_minus_j = BigUint::one();
        for j in 0..m {
            let giant_step = (&values.a * &g_to_m_to_minus_j) % &values.p;

            if let Some(&i) = baby_steps.get(&giant_step) {
                let r = i as u64 + j as u64 * m as u64;

                // verify r is in valid range
                if r < (1u64 << 50) {
                    // verify that g^r ≡ a (mod p)
                    let verification = values.g.modpow(&BigUint::from(r), &values.p);
                    if verification == values.a {
                        return Some(r);
                    }
                }
            }

            g_to_m_to_minus_j = (&g_to_m_to_minus_j * &g_to_minus_m) % &values.p;
        }

        return None;
    }

    // parallel solution
    let chunk_size = 1_000_000u32;
    let num_chunks = (m + chunk_size - 1) / chunk_size;

    (0..num_chunks).into_par_iter().find_map_any(|chunk_idx| {
        let chunk_start = chunk_idx * chunk_size;
        let chunk_end = ((chunk_idx + 1) * chunk_size).min(m);

        // starting value g^(-chunk_start * m)
        let mut g_to_m_to_minus_j = g_to_minus_m.modpow(&BigUint::from(chunk_start), &values.p);

        for j in chunk_start..chunk_end {
            let giant_step = (&values.a * &g_to_m_to_minus_j) % &values.p;

            if let Some(&i) = baby_steps.get(&giant_step) {
                let r = i as u64 + j as u64 * m as u64;

                // verify r is in valid range
                if r < (1u64 << 50) {
                    // verify that g^r ≡ a (mod p)
                    let verification = values.g.modpow(&BigUint::from(r), &values.p);
                    if verification == values.a {
                        return Some(r);
                    }
                }
            }

            g_to_m_to_minus_j = (&g_to_m_to_minus_j * &g_to_minus_m) % &values.p;
        }
        None
    })
}

/*
   extended Euclidean
   ax ≡ 1 (mod m)
   equivalent to ax + my = 1

   Bézout's Identity: gcd(a, b) = a * x + b * y

   if gcd(a, m) = 1, then a has a modular inverse modulo m

    algo:
        gcd(a, b):
            if b = 0: return a
            else: return gcd(b, a mod b)
*/
fn extended_gcd(a: &BigUint, b: &BigUint) -> (BigUint, BigInt, BigInt) {
    // gcd(a, 0) = a = 1*a + 0*b
    if b.is_zero() {
        return (a.clone(), BigInt::one(), BigInt::zero());
    }

    let mut old_remainder = a.clone();
    let mut remainder = b.clone();

    let mut old_a_coeff = BigInt::one();
    let mut old_b_coeff = BigInt::zero();
    let mut curr_a_coeff = BigInt::zero();
    let mut curr_b_coeff = BigInt::one();

    while !remainder.is_zero() {
        let quotient = BigInt::from(&old_remainder / &remainder);
        let new_remainder = &old_remainder - &quotient.to_biguint().unwrap() * &remainder;

        let new_a_coeff = &old_a_coeff - &quotient * &curr_a_coeff;
        let new_b_coeff = &old_b_coeff - &quotient * &curr_b_coeff;

        old_remainder = remainder;
        remainder = new_remainder;

        old_a_coeff = curr_a_coeff;
        old_b_coeff = curr_b_coeff;

        curr_a_coeff = new_a_coeff;
        curr_b_coeff = new_b_coeff;
    }

    (old_remainder, old_a_coeff, old_b_coeff)
}

fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let (gcd, x, _) = extended_gcd(a, m);

    if !gcd.is_one() {
        return None;
    }

    // handle negative x
    let result = match x.sign() {
        Sign::Plus => x.to_biguint().unwrap() % m,
        Sign::Minus => {
            let abs_x = x.magnitude();
            m - (abs_x % m)
        }
        Sign::NoSign => BigUint::zero(),
    };

    Some(result)
}

fn calculate_x(r: u64, values: &ProtocolValues) -> BigUint {
    let r_big = BigUint::from(r);
    let phi_p = &values.p - BigUint::from(1u64); // phi_p = p - 1 since p is prime

    // Calculate (t - r), handling underflow
    let t_minus_r = if values.t >= r_big {
        &values.t - &r_big
    } else {
        &phi_p - ((&r_big - &values.t) % &phi_p)
    };

    let t_minus_r_phi = &t_minus_r % &phi_p;

    // t ≡ r + cx (mod phi_p)
    if let Some(c_inverse_phi) = mod_inverse(&values.c, &phi_p) {
        return (&t_minus_r_phi * &c_inverse_phi) % &phi_p;
    }

    println!("c and phi_p not coprime");

    // fallback in case phi_p and c are not coprimes
    // check if (t - r) is divisible by c as integers
    if &t_minus_r % &values.c == BigUint::zero() {
        println!("(t - r) is divisible by c");
        let x = &t_minus_r / &values.c;
        return x % &values.p;
    }

    panic!("c is not invertible modulo phi_p and (t-r) not divisible by c");
}

pub fn run(filename: &str, use_parallel: bool) {
    let protocol_values = parse_input(filename).expect("Failed to parse input");

    println!("Running with parallelization: {}", use_parallel);

    let start_time = Instant::now();
    let r = find_r_bsgs(&protocol_values, use_parallel).expect("Failed to find r");
    let elapsed = start_time.elapsed();

    println!("Time to find r: {:?}", elapsed);

    let x = calculate_x(r, &protocol_values);

    println!("r: {}", r);
    println!("x: {}", x);

    println!("Verification:");

    // g^r ≡ a (mod p)
    let a_check = protocol_values
        .g
        .modpow(&BigUint::from(r), &protocol_values.p);
    println!("g^r ≡ a (mod p): {}", a_check == protocol_values.a);

    // h = g^x
    let h_check = protocol_values.g.modpow(&x, &protocol_values.p);
    println!("h = g^x: {}", h_check == protocol_values.h);

    // g^t ≡ a * h^c (mod p)
    let left_side = protocol_values
        .g
        .modpow(&protocol_values.t, &protocol_values.p);
    let right_side = (&protocol_values.a
        * &protocol_values
            .h
            .modpow(&protocol_values.c, &protocol_values.p))
        % &protocol_values.p;
    println!("g^t ≡ a * h^c (mod p): {}", left_side == right_side);
}
