pub fn merge_sort<T>(mut vector: Vec<T>) -> Vec<T>
where T: std::cmp::PartialOrd + std::fmt::Debug
{
    // sort the left half,
    // sort the right half, => O(n * ln(n))
    // bring the sorted halves together, => O(n)

    if vector.len() <= 1 {
        return vector;
    };

    let mut res = Vec::with_capacity(vector.len());
    let mut b = vector.split_off(vector.len() / 2);
    let a = merge_sort(vector);
    b = merge_sort(b);

    // bring them together again
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();

    let a_peek = a_it.next();
    let b_peek = b_it.next();

    loop {
        match a_peak {
            Some(ref a_val) => {},
            None => {},
        }
    }

    return vector;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {

    }
}