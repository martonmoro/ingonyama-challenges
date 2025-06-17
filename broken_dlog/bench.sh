#!/bin/bash

echo "bigint sequential:" | tee -a bench_results.txt
cargo run --release bigint false values.txt | grep "Time to find r" | tee -a bench_results.txt

echo "bigint parallel:" | tee -a bench_results.txt
cargo run --release bigint true values.txt | grep "Time to find r" | tee -a bench_results.txt

echo "rug sequential:" | tee -a bench_results.txt
cargo run --release rug false values.txt | grep "Time to find r" | tee -a bench_results.txt

echo "rug parallel:" | tee -a bench_results.txt
cargo run --release rug true values.txt | grep "Time to find r" | tee -a bench_results.txt