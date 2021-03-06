use std::fs;

pub struct InstructionSet {
    opcode: u8,
    param1_mode: u8,
    param2_mode: u8,
}

impl Default for InstructionSet {
    fn default() -> Self {
        InstructionSet {
            opcode: 0,
            param1_mode: 0,
            param2_mode: 0,
        }
    }
}

impl InstructionSet {
    fn new_raw_code(&mut self, raw: i32) {
        // Reset default values.
        self.opcode = 0;
        self.param1_mode = 0;
        self.param2_mode = 0;

        // Parse vec of integers from raw instruction-set code integer.
        let digit_chars: Vec<char> = raw.to_string().chars().collect::<Vec<_>>();
        let digits: Vec<u8> = digit_chars
            .iter()
            .map(|c| *c as u8 - 48) // -48 = integer represented by ascii char value
            .collect::<Vec<u8>>();
        let count: usize = digits.iter().count();

        // Parse opcode.
        if count == 1 && (digits[0] > 0 && digits[0] < 9) {
            self.opcode = digits[0];
        } else if digits[count - 2] == 0 && (digits[count - 1] > 0 && digits[count - 1] < 9) {
            self.opcode = digits[count - 1];
        } else if digits[count - 2] == 9 && digits[count - 1] == 9 {
            self.opcode = 99;
        } else {
            panic!("Error! Unable to parse valid opcode from {}", raw);
        }

        // Parse paramater modes, if specified in raw instruction-set code integer.
        let check_param_mode = |n: u8| {
            if n != 0 && n != 1 {
                panic!("Error! Unable to parse valid parameter mode from {}", raw);
            }
        };

        if count > 2 {
            self.param1_mode = digits[count - 3];
            check_param_mode(self.param1_mode);

            if count > 3 {
                self.param2_mode = digits[count - 4];
                check_param_mode(self.param2_mode);
            }
        }
    }
}

fn main() {
    let memory: Vec<i32> = parse_memory_from_text_file("input.txt");
    let mut highest_output: i32 = 0;

    let permutations: Vec<Vec<i32>> = find_permutations(vec![0, 1, 2, 3, 4]);

    for p in permutations {
        let output: i32 = run_amplifiers(memory.clone(), vec![p[0], p[1], p[2], p[3], p[4]]);

        if output > highest_output {
            highest_output = output;
        }
    }

    println!("output: {:?}", highest_output);
}

// Non-recursive version of Heap's Algorithm, adapted from en.wikipedia.org/wiki/Heap's_algorithm
fn find_permutations(mut array: Vec<i32>) -> Vec<Vec<i32>> {
    let n: usize = array.iter().count();
    let mut permutations: Vec<Vec<i32>> = vec![];

    //c is an encoding of the stack state.
    let mut c: Vec<usize> = vec![0, 0, 0, 0, 0];

    permutations.push(array.clone());

    //i acts similarly to the stack pointer
    let mut i: usize = 0;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                let temp = array[0];
                array[0] = array[i];
                array[i] = temp;
            } else {
                let temp = array[c[i]];
                array[c[i]] = array[i];
                array[i] = temp;
            }

            permutations.push(array.clone());
            //Swap has occurred ending the for-loop. Simulate the increment of the for-loop counter
            c[i] += 1;
            //Simulate recursive call reaching the base case by bringing the pointer to the base case analog in the array
            i = 0;
        } else {
            //The 'for-loop' terminated. Reset the state and simulate popping the stack by incrementing the pointer.
            c[i] = 0;
            i += 1;
        }
    }

    permutations
}

fn run_amplifiers(memory: Vec<i32>, phases: Vec<i32>) -> i32 {
    let mut input: i32 = 0;
    let mut output: i32 = 0;

    for i in 0..5 {
        output = run_program(memory.clone(), input, phases[i]);
        input = output;
    }

    output
}

fn run_program(mut memory: Vec<i32>, input: i32, phase: i32) -> i32 {
    let mut pointer: usize = 0;
    let mut intcode: InstructionSet = Default::default();
    let mut output: i32 = 0;
    let mut phase_set: bool = false;

    loop {
        intcode.new_raw_code(memory[pointer]);

        if intcode.opcode == 1 {
            let param_1 = get_value(&intcode.param1_mode, &(pointer + 1), &memory);
            let param_2 = get_value(&intcode.param2_mode, &(pointer + 2), &memory);
            let address = memory[pointer + 3] as usize;

            memory[address] = param_1 + param_2;
            pointer += 4;
        } else if intcode.opcode == 2 {
            let param_1 = get_value(&intcode.param1_mode, &(pointer + 1), &memory);
            let param_2 = get_value(&intcode.param2_mode, &(pointer + 2), &memory);
            let address = memory[pointer + 3] as usize;

            memory[address] = param_1 * param_2;
            pointer += 4;
        } else if intcode.opcode == 3 {
            let address = memory[pointer + 1] as usize;

            if phase_set == false {
                memory[address] = phase;
                phase_set = true;
            } else {
                memory[address] = input;
            }

            pointer += 2;
        } else if intcode.opcode == 4 {
            let param_1 = get_value(&intcode.param1_mode, &(pointer + 1), &memory);

            output = param_1;
            pointer += 2;
        } else if intcode.opcode == 5 {
            let param_1 = get_value(&intcode.param1_mode, &(pointer + 1), &memory);

            if param_1 != 0 {
                let param_2 = get_value(&intcode.param2_mode, &(pointer + 2), &memory);
                pointer = param_2 as usize;
            } else {
                pointer += 3
            }
        } else if intcode.opcode == 6 {
            let param_1 = get_value(&intcode.param1_mode, &(pointer + 1), &memory);

            if param_1 == 0 {
                let param_2 = get_value(&intcode.param2_mode, &(pointer + 2), &memory);
                pointer = param_2 as usize;
            } else {
                pointer += 3
            }
        } else if intcode.opcode == 7 {
            let param_1 = get_value(&intcode.param1_mode, &(pointer + 1), &memory);
            let param_2 = get_value(&intcode.param2_mode, &(pointer + 2), &memory);
            let address = memory[pointer + 3] as usize;

            if param_1 < param_2 {
                memory[address] = 1;
            } else {
                memory[address] = 0;
            }

            pointer += 4;
        } else if intcode.opcode == 8 {
            let param_1 = get_value(&intcode.param1_mode, &(pointer + 1), &memory);
            let param_2 = get_value(&intcode.param2_mode, &(pointer + 2), &memory);
            let address = memory[pointer + 3] as usize;

            if param_1 == param_2 {
                memory[address] = 1;
            } else {
                memory[address] = 0;
            }

            pointer += 4;
        } else if intcode.opcode == 99 {
            break;
        }
    }

    output
}

// Returns either the value in 'memory' at 'pointer' index, or the value at the 'memory' index given
// by the value in 'memory' at 'pointer' index, depending on the value of 'param_mode' (0 or 1).
fn get_value(param_mode: &u8, pointer: &usize, memory: &Vec<i32>) -> i32 {
    let val_or_posn = memory[*pointer];

    if param_mode == &0 {
        return memory[val_or_posn as usize] as i32;
    } else {
        return val_or_posn as i32;
    }
}

fn parse_memory_from_text_file(filename: &str) -> Vec<i32> {
    let memory_string: String = fs::read_to_string(filename).expect("Error reading file!");
    let split_input: Vec<&str> = memory_string.split(',').collect();
    let memory: Vec<i32> = split_input
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    memory
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
    }

    #[test]
    #[should_panic(expected = "Error! Unable to parse valid parameter mode from 1302")]
    fn invalid_param_mode() {
        let mut intcode: InstructionSet = Default::default();

        intcode.new_raw_code(1302);
    }

    #[test]
    fn run_amplifiers_test() {
        let memory: Vec<i32> = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let phases: Vec<i32> = vec![4, 3, 2, 1, 0];
        let output: i32 = run_amplifiers(memory, phases);
        assert_eq!(output, 43210);

        let memory: Vec<i32> = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let phases: Vec<i32> = vec![0, 1, 2, 3, 4];
        let output: i32 = run_amplifiers(memory, phases);
        assert_eq!(output, 54321);

        let memory: Vec<i32> = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let phases: Vec<i32> = vec![1, 0, 4, 3, 2];
        let output: i32 = run_amplifiers(memory, phases);
        assert_eq!(output, 65210);
    }
}
