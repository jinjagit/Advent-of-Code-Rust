fn main() {
    let intcode = vec![1, 2, 3, 4, 5, 6, 7 , 8];

    run_all_intcode_blocks(&intcode);

    println!("intcode in main: {:?}", intcode);
}


fn run_intcode_block(intcode: &Vec<u32>, code_block: &[u32]) {
    let intcode_iter = intcode.iter();

    for i in intcode_iter {
        println!("i: {}", i);
    }

    for i in 0..4 {
        println!("code_block i: {} = {}", i, code_block[i]);
    }
}

fn run_all_intcode_blocks(intcode: &Vec<u32>) {
    let mut counter: usize = 0;

    loop {
        let start = counter * 4;

        if let Some(_value) = &intcode.get(start) {
            let code_block = &intcode[start..start + 4];
            run_intcode_block(intcode, code_block);
        } else {
            break;
        }

        counter += 1;
    }
}
