module modulo_compare (
    input wire clk,
    input wire reset,
    input wire [299:0] X,
    output wire [255:0] O_trivial,
    output wire [255:0] O_barrett
);

modulo_machine_trivial trivial_inst (
    .clk(clk),
    .reset(reset),
    .X(X),
    .O(O_trivial)
);

modulo_machine_barrett barrett_inst (
    .clk(clk),
    .reset(reset),
    .X(X),
    .O(O_barrett)
);

endmodule