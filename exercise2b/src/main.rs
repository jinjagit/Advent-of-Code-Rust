fn main() {
    let mut intcode = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 2, 19, 6, 23, 2, 13, 23, 27,
        1, 9, 27, 31, 2, 31, 9, 35, 1, 6, 35, 39, 2, 10, 39, 43, 1, 5, 43, 47, 1, 5, 47, 51, 2, 51,
        6, 55, 2, 10, 55, 59, 1, 59, 9, 63, 2, 13, 63, 67, 1, 10, 67, 71, 1, 71, 5, 75, 1, 75, 6,
        79, 1, 10, 79, 83, 1, 5, 83, 87, 1, 5, 87, 91, 2, 91, 6, 95, 2, 6, 95, 99, 2, 10, 99, 103,
        1, 103, 5, 107, 1, 2, 107, 111, 1, 6, 111, 0, 99, 2, 14, 0, 0,
    ];

    let mut noun: u32 = 0;
    let mut verb: u32 = 0;
    let mut result_found: bool = false;

    for i in 0..100 {
        for j in 0..100 {
            intcode[1] = i;
            intcode[2] = j;
            let resultant_code = run_all_intcode_blocks(&intcode);

            if resultant_code[0] == 19690720 {
                noun = i;
                verb = j;
                result_found = true;
                break;
            }
        }

        if result_found == true {
            break;
        }
    }

    println!("result found: {}", result_found);
    println!("verb: {}", verb);
    println!("noun: {}", noun);
    println!("input (100 * noun + verb): {}", 100 * noun + verb);
}

fn run_all_intcode_blocks(intcode: &Vec<u32>) -> Vec<u32> {
    let mut counter: usize = 0;

    let mut new_intcode = intcode.to_vec();

    loop {
        let start = counter * 4;

        if let Some(99) = &intcode.get(start) {
            break;
        } else if let Some(_value) = &intcode.get(start) {
            let code_block = &intcode[start..start + 4];
            new_intcode = run_intcode_block(&new_intcode, code_block);
        } else {
            println!("Error! No exit code '99' encountered");
            break;
        }

        counter += 1;
    }

    new_intcode
}

fn run_intcode_block(intcode: &Vec<u32>, code_block: &[u32]) -> Vec<u32> {
    let mut new_intcode = intcode.to_vec();

    if code_block[0] == 1 {
        new_intcode[code_block[3] as usize] =
            new_intcode[code_block[1] as usize] + new_intcode[code_block[2] as usize];
    } else if code_block[0] == 2 {
        new_intcode[code_block[3] as usize] =
            new_intcode[code_block[1] as usize] * new_intcode[code_block[2] as usize];
    }

    new_intcode
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_all_intcode_blocks_test() {
        let intcode = vec![1, 0, 0, 0, 99];
        let result = run_all_intcode_blocks(&intcode);
        assert_eq!(result, [2, 0, 0, 0, 99]);

        let intcode = vec![2, 3, 0, 3, 99];
        let result = run_all_intcode_blocks(&intcode);
        assert_eq!(result, [2, 3, 0, 6, 99]);

        let intcode = vec![2, 4, 4, 5, 99, 0];
        let result = run_all_intcode_blocks(&intcode);
        assert_eq!(result, [2, 4, 4, 5, 99, 9801]);

        let intcode = vec![1, 1, 1, 4, 2, 5, 6, 0, 99];
        let result = run_all_intcode_blocks(&intcode);
        assert_eq!(result, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
