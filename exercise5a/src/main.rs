// Summary of ship computer functionality:

// code executed in blocks (no longer always of 4 elements)
// 1st instruction contains opcode (as last 2 digits):

// opcodes:
// 01 = add 2 parameters & return result
// 02 = multiply 2 parameters & return result
// 03 = save an input parameter
// 04 = output a parameter
// 99 = end program

// First instruction:
// ABCDE
//  1002

// DE - two-digit opcode,      02 == opcode 2
//  C - mode of 1st parameter,  0 == position mode
//  B - mode of 2nd parameter,  1 == immediate mode
//  A - mode of 3rd parameter,  0 == position mode,
//                                   omitted due to being a leading zero

// parameter modes:
// 0 = position (parameter is interpreted as a value)
// 1 = immediate (parameter is interpreted as a value)

// -------------------------------------------------------------------------------------------------
// Parameters that an instruction writes to will never be in immediate mode.
//   What does this mean?
//   I think we can consider opcodes 01 & 02 as ALWAYS writing to a parameter. 
//   Thus, their 3rd parameter will always be position mode (0).

// xyz01, 1, 2, 2 => opcode 1, R, R, W (x will not be immediate mode == position == 0)
// R's (reads) could be immediate values or values at positions (depending on parameter modes y & z)

// I am also assuming that there will only be one input and one output instruction encountered,
// otherwise I am lost!
// If this is true, then we do not consider output as 'writing', but reading (either a position or
// an immediate value). Input probably also must be in position mode (it is 'given' a value to put
// somewhere in 'memory')
// -------------------------------------------------------------------------------------------------

// It is important to remember that the instruction pointer should increase by the number of values
// in the instruction after the instruction finishes. Because of the new instructions,
// this amount is no longer always 4. (opcodes 3 & 4 result in + 1 value? = + 2 pointer?)
//
// Integers can be negative: 1101,100,-1,4,0 is a valid program
// (find 100 + -1, store the result in position 4).

// The program 3,0,4,0,99 outputs whatever it gets as input, then halts.

// opcode 03 will save whatever it gets as input at position 0, opcode 04 (3rd element) will ouput
// whatever has been given as an input, at position 0. Opcode 99 then terminates the program.



fn main() {
    println!("Hello, world!");
}
