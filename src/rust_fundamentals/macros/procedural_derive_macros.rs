use x_data::{StylableMacro};

pub trait Stylable {
    fn restyle();
}

fn main() {
    Food::restyle();
}

#[derive(StylableMacro)]
pub struct Food {
    name: String,
}

#[cfg(test)]
mod tests {
    fn test() {
        assert_eq!(2+ 2, 4);
    }
}