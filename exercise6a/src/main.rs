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
    let memory_string: String = fs::read_to_string("input.txt").expect("Error reading file!");
    let split_input: Vec<&str> = memory_string.split('\n').collect();
    let mut input: Vec<&str> = vec![];
    let mut index: usize = 0;

    for _elem in &split_input {
        let new_elem: Vec<&str> = split_at_parenthesis(&split_input[index]);
        input.push(new_elem[0]);
        input.push(new_elem[1]);
        index += 1;
    }

    println!("input vec: {:?}", input);



    
    let mut my_vec: Vec<Body> = vec![];

    for i in 0..5 {
        let my_struct = Body {
            name: "some_name",
            orbits: i,
            orbiting: "some_name", 
        };

        my_vec.push(my_struct);
    }

    // for elem in my_vec {
    //     elem.print();
    //     println!("name: {}", elem.get_name());
    // }
    
}

fn split_at_parenthesis(string: &str) -> Vec<&str> {
    let mut list = vec![];

    for elem in string.split(")").filter(|&x| !x.is_empty()) {
        list.push(elem);
    }

    list
}
