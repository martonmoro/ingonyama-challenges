module modulo_machine_barrett (
    input wire clk,
    input wire reset,
    input wire [299:0] X,
    output reg [255:0] O
);

// P = 104899928942039473597645237135751317405745389583683433800060134911610808289117
parameter [255:0] P = 256'hE7EB417862865B8FF6FA5C28E93008D69368F209AD2757CC370682FE26BDC75D;

// R = floor(2^512 / P)
parameter [256:0] R = 257'h11a94da6b4beeafe5851e6105114f39f593a0578c5d889b4e670b5c00558f50e9;

wire [556:0] x_times_r;
wire [44:0] q;
wire [300:0] q_times_p;  // 45 + 256 = 301 bits max
wire [299:0] remainder;
wire [255:0] reduced_once;
wire [255:0] reduced_twice;

assign x_times_r = X * R;
assign q = x_times_r[556:512];  // extract 45-bit quotient
assign q_times_p = q * P;  // 45-bit * 256-bit
assign remainder = X - q_times_p[299:0];

assign reduced_once = (remainder >= {44'd0, P}) ? remainder[255:0] - P : remainder[255:0];

assign reduced_twice = (reduced_once >= P) ? reduced_once - P : reduced_once;

always @(posedge clk or posedge reset) begin
    if (reset) begin
        O <= 256'd0;
    end else begin
        O <= reduced_twice;
    end
end

endmodule