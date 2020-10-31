use std::fs;

fn main() {
    let digits: Vec<u8> = parse_digits_from_input("input.txt");
    let layers: Vec<Vec<u8>> = parse_layers_from_digits(digits, 25, 6);
    let image: Vec<u8> = parse_image_data(layers);

    print_image(image, 25, 6);
}

fn print_image(image_data: Vec<u8>, width: usize, height: usize) {
    for i in 0..height {
        for j in 0..width {
            if image_data[i * width + j] == 0 {
                print!(" ");
            } else {
                print!("#");
            }
        }

        print!("\n");
    }
}

fn parse_image_data(layers: Vec<Vec<u8>>) -> Vec<u8> {
    let mut image: Vec<u8> = vec![];

    for i in 0..layers[0].iter().count() {
        let mut pixel: u8 = 2;

        for j in 0..layers.iter().count() {
            if layers[j][i] == 1 || layers[j][i] == 0 {
                pixel = layers[j][i];
                break;
            }
        }

        image.push(pixel);
    }

    image
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

        let digits = vec![0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0];
        let layers: Vec<Vec<u8>> = parse_layers_from_digits(digits, 2, 2);
        assert_eq!(
            layers,
            [[0, 2, 2, 2], [1, 1, 2, 2], [2, 2, 1, 2], [0, 0, 0, 0]]
        );
    }

    #[test]
    fn parse_image_data_test() {
        let layers: Vec<Vec<u8>> = vec![
            vec![0, 2, 2, 2],
            vec![1, 1, 2, 2],
            vec![2, 2, 1, 2],
            vec![0, 0, 0, 0],
        ];
        let image: Vec<u8> = parse_image_data(layers);
        assert_eq!(image, [0, 1, 1, 0]);
    }
}
