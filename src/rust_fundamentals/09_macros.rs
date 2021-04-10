// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_macro_test() {
        let result = sum_macro!(2, 4, 6);
        assert_eq!(result, 12);
    }

    #[test]
    fn sum_vector_macro() {
        let result = vec_macro!(1, 2, 3, 4, 5);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sum_macro() {
        let mut x = 1;
        summer![x; [2, 34, 5]];
        assert_eq!(x, 42);
    }
}

// Macros
#[macro_export]
macro_rules! summer {
    ($x: ident; $($num: expr), *) => {
        $(
            // $x += $num;
            println!("{:?}", $num);
            for i in $num.iter() {
                $x += i;
            }
        )*
    }
}

#[macro_export]
macro_rules! vec_macro {
    ( $($x: expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($x); )*
            temp_vec
        }
    }
}

#[macro_export]
macro_rules! sum_macro {
    ( $($x: expr), +) => {
        {
            let mut acc = 0;
            $( acc += $x; )+
            acc
        }
    }
}
