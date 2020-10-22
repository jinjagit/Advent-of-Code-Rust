fn main() {
    let intcode = vec![1, 1, 1, 4, 2, 5, 6, 0, 99];

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
            let code_block = &intcode[start..start + 4];

            println!("block: {:?}", code_block);

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
        let intcode = vec![1,0,0,0,99];
        let result = run_all_intcode_blocks(&intcode);
        assert_eq!(result, [2,0,0,0,99]);

        let intcode = vec![2,3,0,3,99];
        let result = run_all_intcode_blocks(&intcode);
        assert_eq!(result, [2,3,0,6,99]);

        let intcode = vec![2,4,4,5,99,0];
        let result = run_all_intcode_blocks(&intcode);
        assert_eq!(result, [2,4,4,5,99,9801]);

        let intcode = vec![1, 1, 1, 4, 2, 5, 6, 0, 99];
        let result = run_all_intcode_blocks(&intcode);
        assert_eq!(result, [30,1,1,4,2,5,6,0,99]);
    }
}
