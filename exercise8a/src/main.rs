use std::fs;

fn main() {
    let digits: Vec<u8> = parse_digits_from_input("input.txt");
    let layers: Vec<Vec<u8>> = parse_layers_from_digits(digits, 25, 6);

    let mut count: i32 = -1;
    let mut layer_num: usize = 0; // layer with fewest 0 digits

    for i in 0..layers.iter().count() {
        let zeroes: i32 = digit_count(layers[i].clone(), 0) as i32;

        if count == -1 || zeroes < count {
            count = zeroes;
            layer_num = i;
        }
    }

    let result: u32 =
        digit_count(layers[layer_num].clone(), 1) * digit_count(layers[layer_num].clone(), 2);

    println!("result: {}", result);
}

fn digit_count(layer: Vec<u8>, digit: u8) -> u32 {
    let mut count: u32 = 0;

    for x in layer {
        if x == digit {
            count += 1;
        }
    }

    count
}

fn parse_layers_from_digits(digits: Vec<u8>, width: usize, height: usize) -> Vec<Vec<u8>> {
    let pixels: usize = width * height;
    let num_layers = digits.iter().count() / pixels;
    let mut layers: Vec<Vec<u8>> = vec![];

    for i in 0..num_layers {
        let mut layer: Vec<u8> = vec![];

        for j in 0..pixels {
            layer.push(digits[i * pixels + j])
        }

        layers.push(layer);
    }

    layers
}

fn parse_digits_from_input(filename: &str) -> Vec<u8> {
    let input_string: String = fs::read_to_string(filename).expect("Error reading file!");

    return input_string
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|c| *c as u8 - 48)
        .collect::<Vec<u8>>();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_layers_from_digits_test() {
        let digits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        let layers: Vec<Vec<u8>> = parse_layers_from_digits(digits, 3, 2);
        assert_eq!(layers, [[1, 2, 3, 4, 5, 6], [7, 8, 9, 0, 1, 2]]);
    }

    #[test]
    fn digit_count_test() {
        let layer = vec![1,2,3,4,1,1,2,3];

        let digit_n = digit_count(layer.clone(), 1);
        assert_eq!(digit_n, 3);

        let digit_n = digit_count(layer, 9);
        assert_eq!(digit_n, 0);
    }
}
