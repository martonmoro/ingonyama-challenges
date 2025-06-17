use rayon::prelude::*;
use rug::Integer;
use std::{collections::HashMap, fs, time::Instant};

struct ProtocolValues {
    p: Integer,
    g: Integer,
    h: Integer,
    a: Integer,
    c: Integer,
    t: Integer,
}

fn parse_input(filename: &str) -> Result<ProtocolValues, Box<dyn std::error::Error>> {
    let input = fs::read_to_string(filename)?;

    let mut values = HashMap::new();
    for line in input.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let value_integer = Integer::from_str_radix(value.trim(), 10)?;
            values.insert(key.trim(), value_integer);
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
    let mut baby_steps: HashMap<Integer, u32> = HashMap::new();
    let m = 1u32 << 25;

    // baby-steps
    let mut g_to_i = Integer::from(1);
    for i in 0..m {
        baby_steps.insert(g_to_i.clone(), i);
        g_to_i = Integer::from(&g_to_i * &values.g) % &values.p;
    }

    // giant-steps
    let g_to_m = values
        .g
        .clone()
        .pow_mod(&Integer::from(m), &values.p)
        .unwrap();
    let p_minus_2 = Integer::from(&values.p - 2u32);
    let g_to_minus_m = g_to_m.clone().pow_mod(&p_minus_2, &values.p).unwrap();

    if !use_parallel {
        // sequential solution
        let mut g_to_m_to_minus_j = Integer::from(1);
        for j in 0..m {
            let giant_step = Integer::from(&values.a * &g_to_m_to_minus_j) % &values.p;

            if let Some(&i) = baby_steps.get(&giant_step) {
                let r = i as u64 + j as u64 * m as u64;

                // verify r is in valid range
                if r < (1u64 << 50) {
                    // verify that g^r ≡ a (mod p)
                    let verification = values
                        .g
                        .clone()
                        .pow_mod(&Integer::from(r), &values.p)
                        .unwrap();
                    if verification == values.a {
                        return Some(r);
                    }
                }
            }

            g_to_m_to_minus_j = Integer::from(&g_to_m_to_minus_j * &g_to_minus_m) % &values.p;
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
        let mut g_to_m_to_minus_j = g_to_minus_m
            .clone()
            .pow_mod(&Integer::from(chunk_start), &values.p)
            .unwrap();

        for j in chunk_start..chunk_end {
            let giant_step = Integer::from(&values.a * &g_to_m_to_minus_j) % &values.p;

            if let Some(&i) = baby_steps.get(&giant_step) {
                let r = i as u64 + j as u64 * m as u64;

                // verify r is in valid range
                if r < (1u64 << 50) {
                    // Verify that g^r ≡ a (mod p)
                    let verification = values
                        .g
                        .clone()
                        .pow_mod(&Integer::from(r), &values.p)
                        .unwrap();
                    if verification == values.a {
                        return Some(r);
                    }
                }
            }

            g_to_m_to_minus_j = Integer::from(&g_to_m_to_minus_j * &g_to_minus_m) % &values.p;
        }
        None
    })
}

fn extended_gcd(a: &Integer, b: &Integer) -> (Integer, Integer, Integer) {
    if *b == 0 {
        return (a.clone(), Integer::from(1), Integer::from(0));
    }

    let mut old_remainder = a.clone();
    let mut remainder = b.clone();
    let mut old_a_coeff = Integer::from(1);
    let mut old_b_coeff = Integer::from(0);
    let mut curr_a_coeff = Integer::from(0);
    let mut curr_b_coeff = Integer::from(1);

    while remainder != 0 {
        let quotient = Integer::from(&old_remainder / &remainder);
        let new_remainder = Integer::from(&old_remainder - &quotient * &remainder);

        let new_a_coeff = Integer::from(&old_a_coeff - &quotient * &curr_a_coeff);
        let new_b_coeff = Integer::from(&old_b_coeff - &quotient * &curr_b_coeff);

        old_remainder = remainder;
        remainder = new_remainder;

        old_a_coeff = curr_a_coeff;
        old_b_coeff = curr_b_coeff;

        curr_a_coeff = new_a_coeff;
        curr_b_coeff = new_b_coeff;
    }

    (old_remainder, old_a_coeff, old_b_coeff)
}

fn mod_inverse(a: &Integer, m: &Integer) -> Option<Integer> {
    let (gcd, x, _) = extended_gcd(a, m);

    if gcd != 1 {
        return None;
    }

    // handle negative x
    let result = if x >= 0 {
        x % m
    } else {
        let abs_x = -x;
        m - (abs_x % m)
    };

    Some(result)
}

fn calculate_x(r: u64, values: &ProtocolValues) -> Integer {
    let r_big = Integer::from(r);
    let phi_p = Integer::from(&values.p - 1u32);

    let t_minus_r = if values.t >= r_big {
        Integer::from(&values.t - &r_big)
    } else {
        let temp = Integer::from(&r_big - &values.t) % &phi_p;
        Integer::from(&phi_p - &temp)
    };

    let t_minus_r_phi = Integer::from(&t_minus_r % &phi_p);

    // t ≡ r + cx (mod phi_p)
    if let Some(c_inverse_phi) = mod_inverse(&values.c, &phi_p) {
        return Integer::from(&t_minus_r_phi * &c_inverse_phi) % &phi_p;
    }

    println!("c and phi_p not coprime");

    // fallback: check if (t - r) is divisible by c
    if Integer::from(&t_minus_r % &values.c) == 0 {
        println!("(t - r) is divisible by c");
        let x = Integer::from(&t_minus_r / &values.c);
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
        .clone()
        .pow_mod(&Integer::from(r), &protocol_values.p)
        .unwrap();
    println!("g^r ≡ a (mod p): {}", a_check == protocol_values.a);

    // h = g^x
    let h_check = protocol_values
        .g
        .clone()
        .pow_mod(&x, &protocol_values.p)
        .unwrap();
    println!("h = g^x: {}", h_check == protocol_values.h);

    // g^t ≡ a * h^c (mod p)
    let left_side = protocol_values
        .g
        .clone()
        .pow_mod(&protocol_values.t, &protocol_values.p)
        .unwrap();
    let h_to_c = protocol_values
        .h
        .clone()
        .pow_mod(&protocol_values.c, &protocol_values.p)
        .unwrap();
    let right_side = Integer::from(&protocol_values.a * &h_to_c) % &protocol_values.p;
    println!("g^t ≡ a * h^c (mod p): {}", left_side == right_side);
}
