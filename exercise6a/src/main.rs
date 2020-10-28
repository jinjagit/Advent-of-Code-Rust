use std::fs;

pub struct Body<'a> {
    name: &'a str,
    orbiting: &'a str,
}

fn main() {
    // Get vec of names (&str's) of orbiting bodies from input text file.
    let input_string: String = fs::read_to_string("input.txt").expect("Error reading file!");
    let split_input: Vec<&str> = input_string.split('\n').collect();
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

    let mut count = bodies.iter().count();
    let mut total_orbits = 0;
    let mut orbits = 1;
    let mut parents: Vec<Body> = vec![];
    let mut children: Vec<Body> = vec![];
    let mut index = count;

    while index > 0 {
        index -= 1;

        if bodies[index].orbiting == "COM" {
            let parent = Body {
                name: bodies[index].name,
                orbiting: bodies[index].orbiting,
            };

            parents.push(parent);
            bodies.remove(index);
            total_orbits += orbits;
            count -= 1;
        }
    }

    while count > 0 {
        orbits += 1;

        for parent in &parents {
            index = count;

            while index > 0 {
                index -= 1;

                if bodies[index].orbiting == parent.name {
                    let child = Body {
                        name: bodies[index].name,
                        orbiting: bodies[index].orbiting,
                    };

                    children.push(child);
                    bodies.remove(index);
                    total_orbits += orbits;
                    count -= 1;
                }
            }
        }

        parents = children;
        children = vec![];
    }

    println!("total orbits: {}", total_orbits);
}
