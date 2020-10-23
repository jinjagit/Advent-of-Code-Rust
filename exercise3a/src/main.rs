// Note: Compile to release build (or will take ages to run with exercise input)
// Takes about 15 seconds to find solution.

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file!");

    let split_input: Vec<&str> = input.split('\n').collect();
    let wire_a: Vec<&str> = split_at_commas(&split_input[0]);
    let wire_b: Vec<&str> = split_at_commas(&split_input[1]);

    let wire_a_coords: Vec<[i32; 2]> = coords_from_vectors(&wire_a);
    let wire_b_coords: Vec<[i32; 2]> = coords_from_vectors(&wire_b);

    let distance = distance_to_closest_shared_point(wire_a_coords, wire_b_coords);

    println!("answer = {}", distance);
}

fn split_at_commas(string: &str) -> Vec<&str> {
    let mut list = vec![];

    for elem in string.split(",").filter(|&x| !x.is_empty()) {
        list.push(elem);
    }

    list
}

fn coords_from_vectors(vectors: &Vec<&str>) -> Vec<[i32; 2]> {
    let mut current_coords: [i32; 2] = [0, 0];
    let mut coords_list: Vec<[i32; 2]> = vec![[0, 0]];

    for vector in vectors {
        let direction: char = vector.chars().nth(0).unwrap();

        let mut steps_string: String = vector.to_string();
        steps_string.replace_range(0..1, "");
        let steps = steps_string.parse::<u32>().unwrap();

        for _i in 0..steps {
            if direction == 'U' {
                current_coords[1] = current_coords[1] + 1;
            } else if direction == 'D' {
                current_coords[1] = current_coords[1] - 1;
            } else if direction == 'L' {
                current_coords[0] = current_coords[0] - 1;
            } else if direction == 'R' {
                current_coords[0] = current_coords[0] + 1;
            }

            coords_list.push(current_coords);
        }
    }

    coords_list
}

fn distance_to_closest_shared_point(coords_a: Vec<[i32; 2]>, coords_b: Vec<[i32; 2]>) -> u32 {
    let mut distance: u32 = 0;

    for point_a in &coords_a {
        for point_b in &coords_b {
            if point_a == point_b {
                let this_distance = (point_a[0].abs() + point_a[1].abs()) as u32;

                if this_distance < distance || distance == 0 {
                    distance = this_distance;
                }
            }
        }
    }

    distance
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_distance_from_vectors_test() {
        let wire_a = vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
        let wire_b = vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];
        let wire_a_coords = coords_from_vectors(&wire_a);
        let wire_b_coords = coords_from_vectors(&wire_b);
        let distance = distance_to_closest_shared_point(wire_a_coords, wire_b_coords);
        assert_eq!(distance, 159);

        let wire_a = vec![
            "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
        ];
        let wire_b = vec![
            "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
        ];
        let wire_a_coords = coords_from_vectors(&wire_a);
        let wire_b_coords = coords_from_vectors(&wire_b);
        let distance = distance_to_closest_shared_point(wire_a_coords, wire_b_coords);
        assert_eq!(distance, 135);
    }
}
