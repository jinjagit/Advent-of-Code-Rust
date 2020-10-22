fn main() {
    let mut intcode = vec![1, 0, 0, 0, 99];

    // "... replace position 1 with the value 12 and replace position 2 with the value 2."
    // intcode[1] = 12;
    // intcode[2] = 2;

    let result = run_all_intcode_blocks(&intcode);

    println!("result: {:?}", result);
}

fn run_all_intcode_blocks(intcode: &Vec<u32>) -> Vec<u32> {
    let mut counter: usize = 0;

    let mut new_intcode = intcode.to_vec();

    loop {
        let start = counter * 4;

        if let Some(99) = &intcode.get(start) {
            break;
        } else if let Some(_value) = &intcode.get(start) {
            //let code_block = &intcode[start..start + 4];
            new_intcode = run_intcode_block(&new_intcode, &start);
        } else {
            println!("Error! No exit code '99' encountered");
            break;
        }

        counter += 1;
    }

    new_intcode
}

fn run_intcode_block(intcode: &Vec<u32>, start: &usize) -> Vec<u32> {
    let mut new_intcode = intcode.to_vec();

    let input_a_index = new_intcode[*start + 1] as usize;
    let input_b_index = new_intcode[*start + 2] as usize;
    let result_index = new_intcode[*start + 3] as usize;

    if new_intcode[*start] == 1 {
        new_intcode[result_index] = new_intcode[input_a_index] + new_intcode[input_b_index];
    } else if new_intcode[*start] == 2 {
        new_intcode[result_index] = new_intcode[input_a_index] * new_intcode[input_b_index];
    }

    println!("4th value: {}", new_intcode[*start + 3]);

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
