
pub struct Body {
    name: String,
    orbits: i32,
    orbiting: String,
}

impl Body {
    fn print(&self) {
        println!("name: {}, orbits: {}, orbiting: {}", self.name, self.orbits, self.orbiting);
    }
}

fn main() {
    let mut my_vec: Vec<Body> = vec![];

    for i in 0..5 {
        let my_struct = Body {
            name: String::from("some_name"),
            orbits: i,
            orbiting: String::from("some_name"), 
        };

        my_vec.push(my_struct);
    }

    for elem in my_vec {
        elem.print();
    }
    
}
