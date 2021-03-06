use std::fs;

pub struct InstructionSet {
    opcode: u8,
    param1_mode: u8,
    param2_mode: u8,
    param3_mode: u8,
}

impl Default for InstructionSet {
    fn default() -> Self {
        InstructionSet {
            opcode: 0,
            param1_mode: 0,
            param2_mode: 0,
            param3_mode: 0,
        }
    }
}

impl InstructionSet {
    fn new_raw_code(&mut self, raw: i64) {
        // Reset default values.
        self.opcode = 0;
        self.param1_mode = 0;
        self.param2_mode = 0;
        self.param3_mode = 0;

        // Parse vec of integers from raw instruction-set code integer.
        let digit_chars: Vec<char> = raw.to_string().chars().collect::<Vec<_>>();
        let digits: Vec<u8> = digit_chars
            .iter()
            .map(|c| *c as u8 - 48) // -48 = integer represented by ascii char value.
            .collect::<Vec<u8>>();
        let count: usize = digits.iter().count();

        // Parse opcode.
        if count == 1 && (digits[0] > 0 && digits[0] < 10) {
            self.opcode = digits[0];
        } else if digits[count - 2] == 0 && (digits[count - 1] > 0 && digits[count - 1] < 10) {
            self.opcode = digits[count - 1];
        } else if digits[count - 2] == 9 && digits[count - 1] == 9 {
            self.opcode = 99;
        } else {
            panic!("Error! Unable to parse valid opcode from {}", raw);
        }

        // Parse paramater modes, if specified in raw instruction-set code integer.
        let check_param_mode = |n: u8| {
            if n > 2 { // Parameter mode can be 0, 1, or 2.
                panic!("Error! Unable to parse valid parameter mode from {}", raw);
            }
        };

        if count > 2 {
            self.param1_mode = digits[count - 3];
            check_param_mode(self.param1_mode);

            if count > 3 {
                self.param2_mode = digits[count - 4];
                check_param_mode(self.param2_mode);

                if count > 4 {
                    self.param3_mode = digits[count - 5];
                    check_param_mode(self.param3_mode);                   
                }
            }
        }
    }
}

fn main() {
    let raw_intcode: Vec<i64> = parse_memory_from_text_file("input.txt");
    let memory: Vec<i64> = add_ram(&raw_intcode);

    // Run program with phase = 1, input = 0, for exercise, as phase is really the 1st input.
    let output = run_program(memory, 0, 2);

    println!("output: {:?}", output);
}

fn parse_memory_from_text_file(filename: &str) -> Vec<i64> {
    let memory_string: String = fs::read_to_string(filename).expect("Error reading file!");
    let split_input: Vec<&str> = memory_string.split(',').collect();
    let memory: Vec<i64> = split_input
        .iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    memory
}

// Increase the length of the input vec by appending a vec of zeroes.
fn add_ram(raw_intcode: &Vec<i64>) -> Vec<i64> {
    let mut memory = raw_intcode.clone();
    let count: usize = raw_intcode.iter().count();
    let mut ram: Vec<i64> = vec![0; count * 8]; // * 8 appears to be a safe minimum, for now.

    memory.append(&mut ram);

    memory
}

fn run_program(mut memory: Vec<i64>, input: i64, phase: i64) -> Vec<i64> {
    let mut pointer: usize = 0;
    let mut intcode: InstructionSet = Default::default();
    let mut output: Vec<i64> = vec![];
    let mut phase_set: bool = false;
    let mut rel_base: i64 = 0;

    loop {
        intcode.new_raw_code(memory[pointer]);

        if intcode.opcode == 1 { // Add p1 + p2, and write to address indicated by p3 (depending on mode).
            let param_1 = get_value(&intcode.param1_mode, &(pointer + 1), &rel_base, &memory);
            let param_2 = get_value(&intcode.param2_mode, &(pointer + 2), &rel_base, &memory);
            let address = get_address(&intcode.param3_mode, &(pointer + 3), &rel_base, &memory);

            memory[address] = param_1 + param_2;
            pointer += 4;
        } else if intcode.opcode == 2 { // Mulitply p1 * p2, and write to address indicated by p3 (depending on mode).
            let param_1 = get_value(&intcode.param1_mode, &(pointer + 1), &rel_base, &memory);
            let param_2 = get_value(&intcode.param2_mode, &(pointer + 2), &rel_base, &memory);
            let address = get_address(&intcode.param3_mode, &(pointer + 3), &rel_base, &memory);

            memory[address] = param_1 * param_2;
            pointer += 4;
        } else if intcode.opcode == 3 { // Input at address indicated by p1 (depending on mode).
            let address = get_address(&intcode.param1_mode, &(pointer + 1), &rel_base, &memory);

            if phase_set == false {
                memory[address] = phase;
                phase_set = true;
            } else {
                memory[address] = input;
            }

            pointer += 2;
        } else if intcode.opcode == 4 { // Output (write to output vec).
            let param_1 = get_value(&intcode.param1_mode, &(pointer + 1), &rel_base, &memory);

            output.push(param_1);
            pointer += 2;
        } else if intcode.opcode == 5 { // If p1 != 0 ? set pointer to p2 value.
            let param_1 = get_value(&intcode.param1_mode, &(pointer + 1), &rel_base, &memory);

            if param_1 != 0 {
                let param_2 = get_value(&intcode.param2_mode, &(pointer + 2), &rel_base, &memory);
                pointer = param_2 as usize;
            } else {
                pointer += 3
            }
        } else if intcode.opcode == 6 { // If p1 == 0 ? set pointer to p2 value.
            let param_1 = get_value(&intcode.param1_mode, &(pointer + 1), &rel_base, &memory);

            if param_1 == 0 {
                let param_2 = get_value(&intcode.param2_mode, &(pointer + 2), &rel_base, &memory);
                pointer = param_2 as usize;
            } else {
                pointer += 3
            }
        } else if intcode.opcode == 7 { // If p1 < p2 ? write 1 : write 0 => to address indicated by p3 (depending on mode).
            let param_1 = get_value(&intcode.param1_mode, &(pointer + 1), &rel_base, &memory);
            let param_2 = get_value(&intcode.param2_mode, &(pointer + 2), &rel_base, &memory);
            let address = get_address(&intcode.param3_mode, &(pointer + 3), &rel_base, &memory);

            if param_1 < param_2 {
                memory[address] = 1;
            } else {
                memory[address] = 0;
            }

            pointer += 4;
        } else if intcode.opcode == 8 { // If p1 == p2 ? write 1 : write 0 => to address indicated by p3 (depending on mode).
            let param_1 = get_value(&intcode.param1_mode, &(pointer + 1), &rel_base, &memory);
            let param_2 = get_value(&intcode.param2_mode, &(pointer + 2), &rel_base, &memory);
            let address = get_address(&intcode.param3_mode, &(pointer + 3), &rel_base, &memory);

            if param_1 == param_2 {
                memory[address] = 1;
            } else {
                memory[address] = 0;
            }

            pointer += 4;
        } else if intcode.opcode == 9 { // Add to relative base.
            let param_1 = get_value(&intcode.param1_mode, &(pointer + 1), &rel_base, &memory);

            rel_base = rel_base + param_1;
            pointer += 2;
        }else if intcode.opcode == 99 { // Exit.
            break;
        }
    }

    output
}

// 'Memory' address used when writing result of opcode 1, 2, 3, 7, or 8.
fn get_address(param_mode: &u8, pointer: &usize, rel_base: &i64, memory: &Vec<i64>) -> usize {
    if param_mode == &2 {
        return (rel_base + memory[*pointer]) as usize;
    } else {
        return memory[*pointer] as usize;
    }
}

// Returns the value at the 'memory' index given by the value in 'memory' at 'pointer' index,
// the value in 'memory' at 'pointer' index, or the value of the 'rel_base',
// depending on the value of 'param_mode' (0, 1, or 2).
fn get_value(param_mode: &u8, pointer: &usize, rel_base: &i64, memory: &Vec<i64>) -> i64 {
    let val_or_posn = memory[*pointer];

    if param_mode == &0 {         // Postion mode
        return memory[val_or_posn as usize] as i64;
    } else if param_mode == &1 {  // Immediate mode
        return val_or_posn as i64;
    } else {                      // Relative mode
        return memory[(rel_base + val_or_posn) as usize] as i64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_opcode_test() {
        let mut intcode: InstructionSet = Default::default();

        intcode.new_raw_code(1002);
        assert_eq!(intcode.opcode, 2);

        intcode.new_raw_code(3);
        assert_eq!(intcode.opcode, 3);

        intcode.new_raw_code(99);
        assert_eq!(intcode.opcode, 99);

        intcode.new_raw_code(199);
        assert_eq!(intcode.opcode, 99);
    }

    #[test]
    #[should_panic(expected = "Error! Unable to parse valid opcode from 1112")]
    fn invalid_opcode() {
        let mut intcode: InstructionSet = Default::default();

        intcode.new_raw_code(1112);
    }

    #[test]
    fn parse_param_modes_test() {
        let mut intcode: InstructionSet = Default::default();

        intcode.new_raw_code(1002);
        assert_eq!(intcode.param1_mode, 0);
        assert_eq!(intcode.param2_mode, 1);

        intcode.new_raw_code(11003);
        assert_eq!(intcode.param1_mode, 0);
        assert_eq!(intcode.param2_mode, 1);

        intcode.new_raw_code(711003);
        assert_eq!(intcode.param1_mode, 0);
        assert_eq!(intcode.param2_mode, 1);

        intcode.new_raw_code(712103);
        assert_eq!(intcode.param1_mode, 1);
        assert_eq!(intcode.param2_mode, 2);
    }

    #[test]
    #[should_panic(expected = "Error! Unable to parse valid parameter mode from 1302")]
    fn invalid_param_mode() {
        let mut intcode: InstructionSet = Default::default();

        intcode.new_raw_code(1302);
    }

    #[test]
    fn run_program_test() {
        // Example 1: Outputs a copy of itself.
        let raw_intcode: Vec<i64> = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let intcode_copy = raw_intcode.clone();
        let memory: Vec<i64> = add_ram(&raw_intcode);
        let output = run_program(memory, 0, 1);
        assert_eq!(output, intcode_copy);

        // Example 2: "... should output a 16-digit number."
        let raw_intcode: Vec<i64> = vec![
            1102, 34915192, 34915192, 7, 4, 7, 99, 0,
        ];
        let memory: Vec<i64> = add_ram(&raw_intcode);
        let output = run_program(memory, 0, 1);
        assert_eq!(output, vec![1219070632396864]);

        // Example 3: "... should output the large number in the middle."
        let raw_intcode: Vec<i64> = vec![
            104, 1125899906842624, 99,
        ];
        let memory: Vec<i64> = add_ram(&raw_intcode);
        let output = run_program(memory, 0, 1);
        assert_eq!(output, vec![1125899906842624]);
    }
}
