// To find the fuel required for a module, take its mass, divide by three,
// round down, and subtract 2.

// Note: The above algo. would give fuel requirement of -1 for a mass of 3!

use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        let mut mass: u32 = 0;

        while mass == 0 {
            print!("Module mass? (Enter a positive integer):");

            let input = get_input();
            mass = string_to_non_zero_u32(input);
        }

        println!("fuel required: {}\n", calculate_fuel(mass));
    }
}

fn get_input() -> String {
    let mut s = String::new();

    let _ = stdout().flush();

    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");

    let char = s.chars().next_back();
    if char == Some('\r') || char == Some('\n') {
        s.pop();
    }

    s
}

fn string_to_non_zero_u32(s: String) -> u32 {
    let i = match s.parse::<u32>() {
        Ok(i) => {
            if i != 0 {
                i
            } else {
                println!("\nERROR! Mass cannot be zero. Try again");
                0
            }
        }
        Err(_) => {
            println!("\nERROR! Try again");
            0
        }
    };

    i
}

fn calculate_fuel(mass: u32) -> u32 {
    let fuel = ((mass as f32 / 3.0).floor() - 2.0) as u32;

    fuel
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calculate_fuel_test() {
        let fuel = calculate_fuel(12);
        assert_eq!(fuel, 2);
        let fuel = calculate_fuel(14);
        assert_eq!(fuel, 2);
        let fuel = calculate_fuel(1969);
        assert_eq!(fuel, 654);
        let fuel = calculate_fuel(100756);
        assert_eq!(fuel, 33583);
    }

    #[test]
    fn string_to_non_zero_u32_test() {
        let result = string_to_non_zero_u32(String::from("not_an_integer"));
        assert_eq!(result, 0);
        let result = string_to_non_zero_u32(String::from(""));
        assert_eq!(result, 0);
        let result = string_to_non_zero_u32(String::from("0"));
        assert_eq!(result, 0);
        let result = string_to_non_zero_u32(String::from("-7"));
        assert_eq!(result, 0);
        let result = string_to_non_zero_u32(String::from("1.14"));
        assert_eq!(result, 0);
        let result = string_to_non_zero_u32(String::from("397"));
        assert_eq!(result, 397);
    }
}
