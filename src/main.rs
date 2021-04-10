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
        let result = vector_test!(1, 2, 3, 4, 5);
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
macro_rules! vector_test {
    ( $($x: expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($x); )*
            temp_vec
        }
    }
}


fn main() {}
//
// #[macro_export]
// macro_rules! vector {
//     ($($item: expr), *) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//               temp_vec.push($item);
//             )*
//             temp_vec
//         }
//     };
//     ($item: expr; $num_copies: expr) => {
//         {
//             let mut temp_vec = Vec::new();
//             temp_vec
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn is_empty() {
//         let result: Vec<i32> = vector![];
//         assert!(result.is_empty());
//     }
//
//     #[test]
//     fn is_work_list() {
//         let result = vector![1, 2, 3];
//         assert_eq!(result, vec![1, 2, 3]);
//         assert_eq!(result.len(), 3);
//     }
// }
