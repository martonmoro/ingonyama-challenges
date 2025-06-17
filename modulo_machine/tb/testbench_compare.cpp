#include <gmpxx.h>
#include <iostream>
#include <random>

#include "Vmodulo_compare.h"
#include "verilated.h"

void tick(Vmodulo_compare *top) {
  top->clk = 0;
  top->eval();
  top->clk = 1;
  top->eval();
}

void set_input(uint32_t *signal, const mpz_class &value, int num_of_bits) {
  int words = (num_of_bits + 31) / 32;
  for (int i = 0; i < words; i++) {
    mpz_class bits32 = (value >> (i * 32)) & 0xFFFFFFFF;
    signal[i] = bits32.get_ui();
  }
}

mpz_class get_output(const uint32_t *signal, int num_of_bits) {
  mpz_class result = 0;
  int words = (num_of_bits + 31) / 32;
  for (int i = words - 1; i >= 0; i--) {
    result <<= 32;
    result |= signal[i];
  }
  mpz_class mask = (mpz_class(1) << num_of_bits) - 1;
  return result & mask;
}

int main(int argc, char **argv) {
  Verilated::commandArgs(argc, argv);

  Vmodulo_compare *top = new Vmodulo_compare;

  gmp_randstate_t rand_state;
  gmp_randinit_default(rand_state);
  gmp_randseed_ui(rand_state, time(NULL));

  mpz_class P("1048999289420394735976452371357513174057453895836834338000601349"
              "11610808289117");

  // reset
  top->reset = 1;
  tick(top);
  top->reset = 0;

  std::cout << "Comparing Trivial vs Barrett Implementation" << std::endl;

  // test values
  mpz_class test_values[] = {mpz_class(0), P, P + 1, P * 2,
                             (mpz_class(1) << 299) - 1};

  bool all_match = true;

  for (int i = 0; i < 5; i++) {
    set_input(top->X, test_values[i], 300);
    tick(top);

    mpz_class trivial_out = get_output(top->O_trivial, 256);
    mpz_class barrett_out = get_output(top->O_barrett, 256);

    if (trivial_out == barrett_out) {
      std::cout << "[PASS] Test " << i + 1 << ": Both outputs match: 0x"
                << trivial_out.get_str(16) << std::endl;
    } else {
      std::cout << "[FAIL] Test " << i + 1 << " outputs differ!" << std::endl;
      std::cout << "  Trivial: 0x" << trivial_out.get_str(16) << std::endl;
      std::cout << "  Barrett: 0x" << barrett_out.get_str(16) << std::endl;
      all_match = false;
    }
  }

  // random tests
  std::cout << "\nRunning 10 random tests" << std::endl;
  for (int i = 0; i < 10; i++) {
    mpz_class X;
    mpz_urandomb(X.get_mpz_t(), rand_state, 300);
    set_input(top->X, X, 300);
    tick(top);

    mpz_class trivial_out = get_output(top->O_trivial, 256);
    mpz_class barrett_out = get_output(top->O_barrett, 256);

    if (trivial_out != barrett_out) {
      std::cout << "[FAIL] Random test " << i + 1 << " failed!" << std::endl;
      all_match = false;
    }
  }

  if (all_match) {
    std::cout << "\n[SUCCESS] All tests passed! Both implementations match."
              << std::endl;
  } else {
    std::cout << "\n[ERROR] Some tests failed!" << std::endl;
  }

  gmp_randclear(rand_state);
  delete top;
  return all_match ? 0 : 1;
}