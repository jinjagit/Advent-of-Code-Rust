
pub struct Body<'a> {
    name: &'a str,
    orbits: i32,
    orbiting: &'a str,
}

impl Body<'_> {
    fn print(&self) {
        println!("name: {}, orbits: {}, orbiting: {}", self.name, self.orbits, self.orbiting);
    }
}

fn main() {
    let mut my_vec: Vec<Body> = vec![];

    for i in 0..5 {
        let my_struct = Body {
            name: "some_name",
            orbits: i,
            orbiting: "some_name", 
        };

        my_vec.push(my_struct);
    }

    for elem in my_vec {
        elem.print();
    }
    
}
