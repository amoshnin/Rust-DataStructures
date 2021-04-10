// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_sum_test() {
        let result = simple_sum!(2, 4, 6);
        assert_eq!(result, 12);
    }

    #[test]
    fn assignment_sum_test() {
        let mut x = 1;
        assignment_sum![x; 2, 34, 5];
        assert_eq!(x, 42);
    }

    #[test]
    fn vector_test() {
        let result = vector!(1, 2, 3, 4, 5);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }
}

// Macros
#[macro_export]
macro_rules! simple_sum {
    ( $($x: expr), +) => {
        {
            let mut acc = 0;
            $( acc += $x; )+
            acc
        }
    }
}

#[macro_export]
macro_rules! assignment_sum {
    ($x: ident; $($num: expr), *) => {
        $(
            $x += $num;
        )*
    }
}

#[macro_export]
macro_rules! vector {
    ( $($x: expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($x); )*
            temp_vec
        }
    }
}
