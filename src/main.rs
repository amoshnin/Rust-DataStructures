
fn main() {

}

#[macro_export]
macro_rules! vector {
    () => {
        {
            let temp_vec = Vec::new();
            temp_vec
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        let result: Vec<i32> = vector![];
        assert!(result.is_empty());
    }
}