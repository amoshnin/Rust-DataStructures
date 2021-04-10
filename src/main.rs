#[macro_export]
macro_rules! vector {
    // Zero or more items with a delimiter of `,` between them //
    ($($item: expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($item);)*
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

#[macro_export]
macro_rules! double_tuple_sum {
    ( $($first_item: expr),*; $($second_item: expr),*) => {
        {
            let mut tuple = (0, 0);
            $( tuple.0 += $first_item; )*
            $( tuple.1 += $second_item; )*
            tuple
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_tuple_sum_test() {
        let result = double_tuple_sum!(1,2,3,4,5; 2,3,4,5,6);
        assert_eq!(result, (15, 20));
    }

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

fn main() {}
