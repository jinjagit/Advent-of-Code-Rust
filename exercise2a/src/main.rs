fn main() {
    let intcode = vec![1, 2, 3, 4, 5, 6, 7 , 8];

    run_all_code_blocks(&intcode);

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

fn run_all_code_blocks(intcode: &Vec<u32>) {
    let code_block = &intcode[0..4];
    
    println!("slice: {:?}", code_block);
    println!("slice[0]: {:?}", code_block[0]);

    run_intcode_block(intcode, code_block);
}
