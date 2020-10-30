use std::fs;

pub struct Body<'a> {
    name: &'a str,
    orbiting: &'a str,
}

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Error reading file!");
    let bodies: Vec<Body> = parse_input(&input_string);
    let transits: i32 = calculate_transits(&bodies);

    println!("transits: {}", transits);
}

fn calculate_transits(bodies: &Vec<Body>) -> i32 {
    // Search through bodies repeatedly, creating a vec of parents of "YOU", and another for "SAN"
    let mut parents_of_start: Vec<Body> = vec_of_parents(&bodies, "YOU");
    let mut parents_of_end: Vec<Body> = vec_of_parents(&bodies, "SAN");

    // Remove all common bodies from 2 vecs, starting at COM (vec[0]).
    loop {
        if parents_of_start[0].name == parents_of_end[0].name { 
            parents_of_start.remove(0);
            parents_of_end.remove(0);
        } else {
            break;
        }
    }

    parents_of_start.iter().count() as i32 + parents_of_end.iter().count() as i32 - 2
}

fn vec_of_parents<'a>(store: &'a Vec<Body>, start: &str) -> Vec<Body<'a>> {
    let mut parents: Vec<Body> = vec![];
    let mut name: &str = start;
    let mut orbiting: &str = "nil";

    loop {
        for body in store {
            if body.name == name {
                orbiting = body.orbiting;

                let body_copy = Body {
                    name: body.name,
                    orbiting: body.orbiting,
                };

                parents.push(body_copy);
                break;
            }
        }

        if orbiting == "COM" { break; }
        name = orbiting;
    }

    return parents.into_iter().rev().collect();
}

fn parse_input(string: &String) -> Vec<Body> {
    // Get vec of names (&str's) of orbiting bodies from input string.
    let split_input: Vec<&str> = string.split('\n').collect();
    let mut input: Vec<&str> = vec![];

    for body_pair in split_input {
        input.append(&mut body_pair.split(")").collect());
    }

    // Create vec of Body structs from input vec.
    let num = input.iter().count();
    let mut bodies: Vec<Body> = vec![];

    for i in 0..num / 2 {
        let body = Body {
            name: input[i * 2 + 1],
            orbiting: input[i * 2],
        };

        bodies.push(body);
    }

    bodies
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_transits_test() {
        let input_string = String::from("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
        let mut bodies: Vec<Body> = parse_input(&input_string);
        let transits: i32 = calculate_transits(&mut bodies);
        assert_eq!(transits, 42);
    }
}
