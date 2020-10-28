use std::fs;

pub struct Body<'a> {
    name: &'a str,
    orbits: i32,
    //orbiting: &'a str,
}

fn main() {
    // Get vec of names (&str's) of orbiting bodies from input text file.
    let input_string: String = fs::read_to_string("input.txt").expect("Error reading file!");
    let split_input: Vec<&str> = input_string.split('\n').collect();
    let mut input: Vec<&str> = vec![];

    for body_pair in split_input {
        input.append(&mut body_pair.split(")").collect());
    }

    // println!("input: {:?}", input);

    for elem in &input {
        if elem == &"COM" {
            println!("Found COM");
        }
    }

    // Create vec of Body structs from input vec, using previously defined structs to find value of
    // orbits for any Body that is not orbiting previous Body in vec.
    // Keep running total of orbits as vec of Body structs is built.

    // DOES NOT WORK, because exercise list does not start with COM, thus all input entries before
    // COM are not totalled properly (orbits not correctly calculated)
    let num = input.iter().count();
    let mut bodies: Vec<Body> = vec![];
    let mut orbits: i32 = 0;
    let mut total_orbits: i32 = 0;

    for i in 0..num / 2 {
        if i == 0 {
            orbits = 1;
        } else if input[i * 2] == input[i * 2 - 1] {
            orbits += 1;
        } else {
            orbits = find_orbits_count(input[i * 2], &bodies) + 1;
        }

        let body = Body {
            name: input[i * 2 + 1],
            orbits: orbits,
            //orbiting: input[i * 2],
        };

        bodies.push(body);
        total_orbits += orbits;
    }

    // for body in bodies {
    //     println! ("name: {}, orbits: {}, orbiting: {}", body.name, body.orbits, body.orbiting);
    // }

    println!("total orbits: {}", total_orbits);
}

fn find_orbits_count(orbiting: &str, bodies: &Vec<Body>) -> i32 {
    let mut orbits: i32 = 0;

    for body in bodies {
        if body.name == orbiting {
            orbits = body.orbits;
        }
    }

    orbits
}
