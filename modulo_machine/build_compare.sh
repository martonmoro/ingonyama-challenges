#!/bin/bash

verilator --cc --exe --build \
    src/modulo_compare.v \
    src/modulo_machine_trivial.v \
    src/modulo_machine_barrett.v \
    tb/testbench_compare.cpp \
    -CFLAGS "$(pkg-config --cflags gmpxx)" \
    -LDFLAGS "$(pkg-config --libs gmpxx)" \
    -o modulo_compare

./obj_dir/modulo_compare