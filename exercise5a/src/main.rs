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
    fn new_raw_code(&mut self, raw: i32) {
        // reset default values
        self.opcode = 0;
        self.param1_mode = 0;
        self.param2_mode = 0;
        self.param3_mode = 0;

        // parse vec of integers from raw code integer
        let digit_chars: Vec<char> = raw.to_string().chars().collect::<Vec<_>>();
        let digits: Vec<u8> = digit_chars
            .iter()
            .map(|c| *c as u8 - 48) // -48 = integer represented by ascii char value
            .collect::<Vec<u8>>();
        let count: usize = digits.iter().count();

        // parse opcode
        if count == 1 && (digits[0] > 0 && digits[0] < 5) {
            self.opcode = digits[0];
        } else if digits[count - 2] == 0 && (digits[count - 1] > 0 && digits[count - 1] < 5) {
            self.opcode = digits[count - 1];
        } else if digits[count - 2] == 9 && digits[count - 1] == 9 {
            self.opcode = 99;
        } else {
            panic!("Error! Unable to parse valid opcode from {}", raw);
        }

        // parse paramater modes, if specified in raw code integer
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
                check_param_mode(self.param1_mode);

                if count > 4 {
                    self.param3_mode = digits[count - 5];
                    check_param_mode(self.param1_mode);
                }
            }
        }
    }
}

fn main() {
    let mut memory: Vec<i32> = parse_memory_from_text_file("input.txt");
    let mut pointer: usize = 0;
    let mut intcode: InstructionSet = Default::default();
    let mut outputs: Vec<i32> = vec![];
    let input: i32 = 1;

    loop {
        intcode.new_raw_code(memory[pointer]);

        if intcode.opcode == 1 {
            let param_1 = get_value(&intcode.param1_mode, &memory[pointer + 1], &memory);
            let param_2 = get_value(&intcode.param2_mode, &memory[pointer + 2], &memory);
            let address = memory[pointer + 3] as usize;
            memory[address] = param_1 + param_2;
            pointer += 4;
        } else if intcode.opcode == 2 {
            let param_1 = get_value(&intcode.param1_mode, &memory[pointer + 1], &memory);
            let param_2 = get_value(&intcode.param2_mode, &memory[pointer + 2], &memory);
            let address = memory[pointer + 3] as usize;
            memory[address] = param_1 * param_2;
            pointer += 4;
        } else if intcode.opcode == 3 {
            let address = memory[pointer + 1] as usize;
            memory[address] = input;
            pointer += 2;
        } else if intcode.opcode == 4 {
            let value: i32 = get_value(&intcode.param1_mode, &memory[pointer + 1], &memory);
            outputs.push(value);
            pointer += 2;
        } else if intcode.opcode == 99 {
            break;
        }
    }

    println!("outputs: {:?}", outputs);
}

fn get_value(param_mode: &u8, val_or_posn: &i32, memory: &Vec<i32>) -> i32 {
    if param_mode == &0 {
        let position = *val_or_posn as usize;
        return memory[position] as i32;
    } else {
        return *val_or_posn;
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
        assert_eq!(intcode.param3_mode, 0);

        intcode.new_raw_code(11003);
        assert_eq!(intcode.param1_mode, 0);
        assert_eq!(intcode.param2_mode, 1);
        assert_eq!(intcode.param3_mode, 1);

        intcode.new_raw_code(711003);
        assert_eq!(intcode.param1_mode, 0);
        assert_eq!(intcode.param2_mode, 1);
        assert_eq!(intcode.param3_mode, 1);
    }

    #[test]
    #[should_panic(expected = "Error! Unable to parse valid parameter mode from 1302")]
    fn invalid_param_mode() {
        let mut intcode: InstructionSet = Default::default();

        intcode.new_raw_code(1302);
    }
}
