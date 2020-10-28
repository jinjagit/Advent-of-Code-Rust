use std::fs;

pub struct Body<'a> {
    name: &'a str,
    orbits: i32,
    orbiting: &'a str,
}

impl Body<'_> {
    fn print(&self) { // DEBUG
        println!("name: {}, orbits: {}, orbiting: {}", self.name, self.orbits, self.orbiting);
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

fn main() {
    // Get vec of names (&str's) of orbiting bodies from input text file.
    let input_string: String = fs::read_to_string("input.txt").expect("Error reading file!");
    let split_input: Vec<&str> = input_string.split('\n').collect();
    let mut input: Vec<&str> = vec![];

    for elem in split_input {
        input.append(&mut elem.split(")").collect());
    }

    println!("input vec: {:?}", input); // DEBUG

    // Next step (in development): produce vec of structs from input vec.
    // Eventual plan: count total_orbits as create vec of structs, using the orbits value of
    // previously added structs as data required when the chain of heirarchy in the input list jumps
    // to a body (struct) that is located more than 1 item previous in the list.
    
    // This is just a POC adding structs to a vec (which can then be read)
    let mut bodies: Vec<Body> = vec![];

    for i in 0..5 {
        let body = Body {
            name: "some_name",
            orbits: i,
            orbiting: "some_name", 
        };

        bodies.push(body);
    }

    // DEBUG:
    // for body in bodies {
    //     body.print();
    //     println!("name: {}", body.get_name());
    // } 
}
