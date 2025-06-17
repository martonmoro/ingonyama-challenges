#!/bin/bash

echo "=== Synthesis Comparison ==="

if command -v yosys &> /dev/null; then
    echo "Synthesizing trivial implementation..."
    yosys -p "read_verilog ../src/modulo_machine_trivial.v; synth; stat" > logs/trivial_stats.txt 2>&1
    
    echo "Synthesizing Barrett implementation..."
    yosys -p "read_verilog ../src/modulo_machine_barrett.v; synth; stat" > logs/barrett_stats.txt 2>&1
    
    echo -e "\nTrivial Implementation Stats:"
    grep -E "(Number of cells|Number of wires|Chip area)" logs/trivial_stats.txt
    
    echo -e "\nBarrett Implementation Stats:"
    grep -E "(Number of cells|Number of wires|Chip area)" logs/barrett_stats.txt
fi

echo -e "\n=== Verilator Statistics ==="
verilator --cc ../src/modulo_machine_trivial.v --stats
echo "Trivial implementation stats saved to logs/trivial_stats.txt"
mv logs/stats.txt logs/trivial_stats.txt 2>/dev/null

verilator --cc ../src/modulo_machine_barrett.v --stats  
echo "Barrett implementation stats saved to logs/barrett_stats.txt"
mv logs/stats.txt logs/barrett_stats.txt 2>/dev/null