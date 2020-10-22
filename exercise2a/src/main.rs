fn main() {
    let intcode = vec![1, 2, 3, 4, 5, 6, 7 , 8, 99, 6, 6, 6];

    run_all_intcode_blocks(&intcode);

    println!("intcode in main: {:?}", intcode);
}


fn run_intcode_block(intcode: &Vec<u32>, code_block: &[u32]) -> Vec<u32> {
    //let new_intcode = &intcode;

    // for i in intcode_iter {
    //     println!("i: {}", i);
    // }

    let mut new_intcode = intcode.to_vec();



    for i in 0..4 {
        println!("code_block i: {} = {}", i, code_block[i]);
    }

    new_intcode[2] = 777;

    new_intcode
}

fn run_all_intcode_blocks(intcode: &Vec<u32>) {
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

    println!("mutated intcode: {:?}", new_intcode);
}

// fn main() {
//     let mut my_vec = vec![1, 2, 3];
//
//     my_vec = mutate_vec(&my_vec);
//
//     println!("my_vec: {:?}", my_vec);
// }
//
// fn mutate_vec(vec: &Vec<u32>) -> Vec<u32> {
//     println!("vec: {:?}", vec);
//
//     let mut new_vec = vec.to_vec();
//
//     new_vec[2] = 777;
//
//     new_vec
// }
