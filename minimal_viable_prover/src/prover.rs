use std::time::Instant;

use crate::setup::TrustedSetup;
use ark_bls12_381::{Fr, G1Projective};
use ark_ec::{CurveGroup, VariableBaseMSM};
use ark_ff::{AdditiveGroup, UniformRand};
use ark_poly::EvaluationDomain;
use rand::thread_rng;
use rayon::prelude::*;

pub struct Prover {
    setup: TrustedSetup,
    blinding_polynomial: Vec<Fr>,
}

impl Prover {
    pub fn new(setup: TrustedSetup) -> Self {
        let blinding_polynomial: Vec<Fr> = (0..setup.domain.size())
            .into_par_iter()
            .map(|_| {
                let mut rng = thread_rng();
                Fr::rand(&mut rng)
            }) // each thread with its own rng
            .collect();

        Prover {
            setup,
            blinding_polynomial,
        }
    }

    pub fn commit(&self, witness: &[Fr]) -> G1Projective {
        let mut padded_witness = witness.to_vec();
        padded_witness.resize(self.setup.domain.size(), Fr::ZERO);

        println!(
            "Witness size: {}, padded to: {}",
            witness.len(),
            padded_witness.len()
        );

        // convert to evaluation form
        let start = Instant::now();
        let witness_evals = self.setup.domain.fft(&padded_witness);
        let fft_time = start.elapsed();
        println!("Witness FFT: {:?}", fft_time);

        // compute Hadamard product
        let start = Instant::now();
        let product: Vec<Fr> = self
            .blinding_polynomial
            .par_iter()
            .zip(witness_evals.par_iter())
            .map(|(blinding, witness)| *blinding * *witness)
            .collect();

        let product_time = start.elapsed();
        println!("Hadamard product: {:?} (parallel)", product_time);

        // MSM
        // sum(product[i] * srs_lagrange[i]) for all i
        let start = Instant::now();
        let commitment = G1Projective::msm(
            &self
                .setup
                .srs_lagrange
                .iter()
                .map(|p| p.into_affine())
                .collect::<Vec<_>>(),
            &product,
        )
        .expect("MSM failed");
        let msm_time = start.elapsed();
        println!("MSM: {:?}", msm_time);

        commitment
    }
}
