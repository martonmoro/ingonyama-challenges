#include <gmpxx.h>

#include <chrono>
#include <iostream>
#include <random>

#include "Vmodulo_machine_trivial.h"
#include "verilated.h"

void tick(Vmodulo_machine_trivial *top) {
  top->clk = 0;
  top->eval();
  top->clk = 1;
  top->eval();
}

void set_input(uint32_t *signal, const mpz_class &value, int num_of_bits) {
  // verilator stores signals as arrays of uint32_t
  int words = (num_of_bits + 32 - 1) / 32; // rounding up
  for (int i = 0; i < words; i++) {
    // extract 32 bits
    mpz_class bits32 = (value >> (i * 32) & 0xFFFFFFFF);

    // set signal
    signal[i] = bits32.get_ui();
  }
}

mpz_class get_output(const uint32_t *signal, int num_of_bits) {
  mpz_class result = 0;
  int words = (num_of_bits + 32 - 1) / 32; // rounding up
  // build from MSB
  for (int i = words - 1; i >= 0; i--) {
    result <<= 32;
    result |= signal[i];
  }

  // only keep `num_of_bits` bits
  mpz_class mask = (mpz_class(1) << num_of_bits) - 1;

  return result & mask;
}

void print_hex(const std::string &label, const mpz_class &val) {
  std::cout << label << " = 0x" << val.get_str(16) << std::endl;
}

void run_test(const std::string &test_name, const mpz_class &X,
              const mpz_class &expected_O, Vmodulo_machine_trivial *top) {
  std::cout << "Testing with " << test_name << "..." << std::endl;

  set_input(top->X, X, 300);
  tick(top);

  mpz_class O = get_output(top->O, 256);

  print_hex("X", X);

  if (O == expected_O) {
    std::cout << "[PASS] O == " << "0x" << expected_O.get_str(16)
              << " as expected." << std::endl;
  } else {
    std::cout << "[FAIL] Expected O == " << "0x" << expected_O.get_str(16)
              << ", but got O = " << O.get_str(16) << std::endl;
  }

  std::cout << std::endl;
}

int main(int argc, char **argv) {
  Verilated::commandArgs(argc, argv);

  Vmodulo_machine_trivial *top = new Vmodulo_machine_trivial;

  // initialize random rand_state
  gmp_randstate_t rand_state;
  gmp_randinit_default(rand_state);        // default algo is LCG
  gmp_randseed_ui(rand_state, time(NULL)); // seed it with current time

  mpz_class P("1048999289420394735976452371357513174057453895836834338000601349"
              "11610808289117");

  top->reset = 1;
  tick(top);
  top->reset = 0;

  // ******************* TESTS *******************

  std::cout << "=== Testing Modulo Machine ===" << std::endl;
  print_hex("P", P);

  // TEST 1: X=0
  run_test("X = 0", 0, 0, top);

  // TEST 2: X=P
  run_test("X = P", P, 0, top);

  // TEST 3: X=P + 1
  run_test("X = P + 1", P + 1, 1, top);

  // TEST 4: random
  mpz_class X;
  mpz_urandomb(X.get_mpz_t(), rand_state, 300);
  mpz_class expected = X % P;
  run_test("X = random 300 bit value", X, expected, top);

  gmp_randclear(rand_state);
  delete top;

  return 0;
}