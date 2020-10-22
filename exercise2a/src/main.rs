fn main() {
    let intcode = vec![1, 2, 3, 4, 5, 6, 7 , 8];

    let slice = &intcode[0..4];

    run_intcode_block(&intcode);

    println!("intcode: {:?}", intcode);
    println!("slice: {:?}", slice);
    println!("slice[0]: {:?}", slice[0]);
}

fn run_intcode_block(intcode: &Vec<u32>) {
    let intcode_iter = intcode.iter();

    for i in intcode_iter {
        println!("i: {}", i);
    }
}
