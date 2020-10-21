// To find the fuel required for a module, take its mass, divide by three,
// round down, and subtract 2.

use std::io::{stdin, stdout, Write};

fn main() {
    let mass: u32 = get_u32();

    println!("Input accepted: {}", mass);
}

fn get_input() -> String {
    let mut s = String::new();

    print!("Module mass? (Enter a positive integer):");

    let _ = stdout().flush();

    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    s
}

fn get_u32() -> u32 {
    let mut i: u32 = 0;

    while i == 0 {
        let input = get_input();

        i = match input.parse::<u32>() {
            Ok(i) => i,
            Err(_) => {
                println!("\nERROR! Try again");
                0
            }
        };
    }

    i
}
