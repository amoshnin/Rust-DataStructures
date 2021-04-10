
fn main() {}

#[macro_export]
macro_rules! vector {
    ($($item: expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
              temp_vec.push($item);
            )*
            temp_vec
        }
    };
    ($item: expr; $num_copies: expr) => {
        {
            let mut temp_vec = Vec::new();
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

    #[test]
    fn is_work_list() {
        let result = vector![1, 2, 3];
        assert_eq!(result, vec![1, 2, 3]);
        assert_eq!(result.len(), 3);
    }
}