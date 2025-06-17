mod prover;
mod setup;

use ark_bls12_381::Fr;
use ark_ff::UniformRand;
use ark_poly::EvaluationDomain;
use ark_std::time::Instant;
use rand::thread_rng;
use std::env;

fn main() {
    let n = 100_000;

    let args: Vec<String> = env::args().collect();
    let use_cache = args.len() > 1 && args[1] == "true";

    println!("Trusted Setup Phase");
    let start = Instant::now();
    let setup = setup::TrustedSetup::new(n, use_cache);
    println!(
        "Working with n = {}, domain size = {}",
        n,
        setup.domain.size()
    );
    println!("Total setup time: {:?}\n", start.elapsed());

    println!("Creating Prover");
    let start = Instant::now();
    let prover = prover::Prover::new(setup);
    println!("Prover creation time: {:?}\n", start.elapsed());

    // generate witness
    let mut rng = thread_rng();

    let witness: Vec<Fr> = (0..n).map(|_| Fr::rand(&mut rng)).collect();

    println!("Computing Commitment");
    let start = Instant::now();
    let commitment = prover.commit(&witness);
    println!("Total commitment time: {:?}", start.elapsed());

    println!("\nCommitment: {:?}", commitment);
}
