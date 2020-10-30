use std::fs;

pub struct Body<'a> {
    name: &'a str,
    orbits: i32,
    orbiting: &'a str,
}

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Error reading file!");
    let mut bodies: Vec<Body> = parse_input(&input_string);
    let total_orbits: i32 = calculate_total_orbits(&mut bodies);

    println!("total orbits: {}", total_orbits);
}

fn calculate_total_orbits(bodies: &mut Vec<Body>) -> i32 {
    let mut count = bodies.iter().count();
    let mut total_orbits = 0;
    let mut orbits = 1;
    let mut parents: Vec<Body> = vec![];
    let mut children: Vec<Body> = vec![];
    let mut store: Vec<Body> = vec![];
    let mut index = count;

    while index > 0 {
        index -= 1;

        if bodies[index].orbiting == "COM" {
            let parent = Body {
                name: bodies[index].name,
                orbits: orbits,
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
                        orbits: orbits,
                        orbiting: bodies[index].orbiting,
                    };

                    children.push(child);
                    bodies.remove(index);
                    total_orbits += orbits;
                    count -= 1;
                }
            }
        }

        store.append(&mut parents);
        parents = children;
        children = vec![];
    }

    store.append(&mut parents);

    // TODO:
    //
    // Search through store repeatedly, creating a vec of refs. to parents of "YOU", and same for "SAN"
    // Use above vecs to calculate answer.

    let mut parents_of_YOU: Vec<&Body> = vec![];
    let mut name: &str = "L";
    let mut orbiting: &str = "nil";

    loop {
        for body in &store {
            if body.name == name {
                orbiting = body.orbiting;
                parents_of_YOU.push(&body);
                break;
            }
        }

        if orbiting == "COM" { break; }
        name = orbiting;
    }

    for body in parents_of_YOU {
        println!("name: {}, orbits: {}, orbiting: {}", body.name, body.orbits, body.orbiting);
    }

    

    total_orbits
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
            orbits: 0,
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
    fn find_total_orbits_test() {
        let input_string = String::from("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
        let mut bodies: Vec<Body> = parse_input(&input_string);
        let total_orbits: i32 = calculate_total_orbits(&mut bodies);
        assert_eq!(total_orbits, 42);
    }
}
