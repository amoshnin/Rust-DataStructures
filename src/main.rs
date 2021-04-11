// Vector generations with Declarative Macros
#[macro_export]
macro_rules! vector {
    // Zero or more items with a delimiter of `,` between them //
    ($($item: expr),* $(,)?) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($item);)*
            temp_vec
        }
    };
    ($item: expr; $count: expr) => {
        {
            let mut temp_vec = Vec::new();
            let x = $item;
            for _ in 0..$count {
                temp_vec.push(x.clone());
            }
            temp_vec
        }
    }
}

// Tuple generations with Declarative Macros
#[macro_export]
macro_rules! double_tuple_sum {
    ( $($first_item: expr),* $(,)?; $($second_item: expr),* $(,)?) => {
        {
            let mut tuple = (0, 0);
            $( tuple.0 += $first_item; )*;
            $( tuple.1 += $second_item; )*;
            tuple
        }
    };
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
    fn vector_empty() {
        let result: Vec<i32> = vector![];
        assert!(result.is_empty());
    }

    #[test]
    fn vector_listing() {
        let result = vector![1, 2, 3];
        assert_eq!(result, vec![1, 2, 3]);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn vector_generation() {
        let mut s = Some(4);
        let result = vector![s.take().unwrap(); 10];
        assert_eq!(result, vec![4; 10]);
    }
}

// Traits implementation with Declarative Macros
trait MaximumValue {
    fn maximum_value() -> Self;
}

macro_rules! max_impl {
    ($type: ty) => {
        impl $crate::MaximumValue for $type {
            fn maximum_value() -> Self {
                <$type>::MAX
            }
        }
    };
}

max_impl!(i32);
max_impl!(u32);
max_impl!(i64);

fn main() {
    let g = i32::maximum_value();
    println!("{}", g);
}
