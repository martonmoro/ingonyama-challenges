use ark_bls12_381::{Fr, G1Projective};
use ark_ec::PrimeGroup;
use ark_ff::{AdditiveGroup, Field, UniformRand};
use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::time::Instant;
use rand::thread_rng;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub struct TrustedSetup {
    pub srs_lagrange: Vec<G1Projective>,
    pub domain: GeneralEvaluationDomain<Fr>,
}

impl TrustedSetup {
    pub fn new(n: usize, use_cache: bool) -> Self {
        let domain = GeneralEvaluationDomain::<Fr>::new(n).expect("Failed to create domain");

        let srs_lagrange = if use_cache {
            Self::get_or_generate_srs(domain.size())
        } else {
            let srs = Self::generate_lagrange_srs(domain.size());
            Self::save_srs(&srs, domain.size());
            srs
        };

        TrustedSetup {
            srs_lagrange,
            domain,
        }
    }

    fn get_or_generate_srs(domain_size: usize) -> Vec<G1Projective> {
        let filename = format!("lagrange_srs_{}_uncompressed.bin", domain_size);

        if let Ok(file) = File::open(&filename) {
            println!("Loading SRS from file...");
            let mut reader = BufReader::new(file);
            let start = Instant::now();

            let srs: Vec<G1Projective> =
                // unchecked version is used beause it skips some validation and results in great speed up
                Vec::<G1Projective>::deserialize_uncompressed_unchecked(&mut reader)
                    .expect("Failed to deserialize");

            let load_time = start.elapsed();
            println!("Loaded SRS in {:?}", load_time);
            return srs;
        }

        println!("Generating new SRS...");
        let srs = Self::generate_lagrange_srs(domain_size);
        Self::save_srs(&srs, domain_size);
        srs
    }

    fn save_srs(srs: &[G1Projective], domain_size: usize) {
        let filename = format!("lagrange_srs_{}_uncompressed.bin", domain_size);
        println!("Saving SRS to file...");
        let file = File::create(&filename).expect("Failed to create file");
        let mut writer = BufWriter::new(file);
        let start = Instant::now();
        srs.serialize_uncompressed(&mut writer)
            .expect("Failed to serialize");
        println!("Saved in {:?}", start.elapsed());
    }

    fn generate_lagrange_srs(domain_size: usize) -> Vec<G1Projective> {
        let mut rng = thread_rng();
        let tau = Fr::rand(&mut rng);

        // generate powers of tau
        let start = Instant::now();
        let mut coeffs = vec![Fr::ZERO; domain_size];
        let mut current_power = Fr::ONE;
        for i in 0..domain_size {
            coeffs[i] = current_power;
            current_power *= tau;
        }
        let powers_time = start.elapsed();

        // create domain for FFT
        let domain =
            GeneralEvaluationDomain::<Fr>::new(domain_size).expect("Failed to create domain");

        // FFT to get evaluations
        let start = Instant::now();
        let evals = domain.fft(&coeffs);
        let fft_time = start.elapsed();

        // scalar multiplications
        let generator = G1Projective::generator();
        let start = Instant::now();
        let srs: Vec<G1Projective> = evals.par_iter().map(|&eval| generator * eval).collect();
        let scalar_mul_time = start.elapsed();

        println!("  Powers generation: {:?}", powers_time);
        println!("  FFT: {:?}", fft_time);
        println!("  Scalar multiplications: {:?} (parallel)", scalar_mul_time);

        srs
    }
}
