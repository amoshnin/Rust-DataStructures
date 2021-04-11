use x_data::{StylableMacro};

pub trait Stylable {
    fn restyle();
}

fn main() {
    Food::restyle();
}

#[derive(StylableMacro)]
struct Food {
    name: String,
}