use std::fs;

pub struct Body<'a> {
    name: &'a str,
    orbits: i32,
    orbiting: &'a str,
}

impl Body<'_> {
    fn print(&self) {
        println!("name: {}, orbits: {}, orbiting: {}", self.name, self.orbits, self.orbiting);
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

fn main() {
    // get vec of components from input
    let input_string: String = fs::read_to_string("input.txt").expect("Error reading file!");
    let split_input: Vec<&str> = input_string.split('\n').collect();
    let mut input: Vec<&str> = vec![];

    for elem in &split_input {
        let mut new_elem: Vec<&str> = elem.split(")").collect();
        input.append(&mut new_elem);
    }

    println!("input vec: {:?}", input); // DEBUG

    // next step (in development): produce vec of structs from input vec
    
    let mut bodies: Vec<Body> = vec![];

    for i in 0..5 {
        let body = Body {
            name: "some_name",
            orbits: i,
            orbiting: "some_name", 
        };

        bodies.push(body);
    }

    // for elem in my_vec {
    //     elem.print();
    //     println!("name: {}", elem.get_name());
    // }
    
}
