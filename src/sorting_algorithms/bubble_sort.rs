// O(n) = Big O = Expected runtime complexity (kind of complexity)
pub fn bubble_sort<T>(list: &mut [T])
where T: std::cmp::PartialOrd + std::fmt::Debug
{
    for num in 0..list.len() {
        let mut sorted = true;
        for i in 0..(list.len() - 1) - num {
            if list[i] > list[i + 1] {
                list.swap(i, i + 1);
                sorted = false;
            }
        }

        println!("{:?}", list);
        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![22, 6, 11, 8, 12, 2, 4];
        bubble_sort(&mut v);
        assert_eq!(v, vec![2, 4, 6, 8, 11, 12, 22]);
    }
}
