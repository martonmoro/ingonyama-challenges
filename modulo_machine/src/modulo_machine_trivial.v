module modulo_machine_trivial (
    input wire clk,
    input wire reset,
    input wire [299:0] X,
    output reg [255:0] O
);

// P = 104899928942039473597645237135751317405745389583683433800060134911610808289117
// P in hex = E7EB417862865B8FF6FA5C28E93008D69368F209AD2757CC370682FE26BDC75D
parameter [299:0] P = 300'hE7EB417862865B8FF6FA5C28E93008D69368F209AD2757CC370682FE26BDC75D;

// Intermediate wire for the modulo result
wire [299:0] mod_result;
assign mod_result = X % P;

always @(posedge clk or posedge reset) begin
    if (reset) begin
        O <= 256'd0;
    end else begin
        // we can safely truncate the result of the modulo division since P is 256 bits long
        O <= mod_result[255:0];
    end
end

endmodule