use std::collections::HashSet;

pub fn two_sum(total: i64, list: &[i64]) -> Option<(i64, i64)> {
    let mut cache = HashSet::new();
    for n in list {
        if let Some(complement) = cache.get(&(total - n)) {
            return Some((*complement, *n));
        }

        cache.insert(*n);
    }

    None
}

pub fn three_sum(total: i64, list: &[i64]) -> Option<(i64, i64, i64)> {
    for (idx, n) in list.iter().enumerate() {
        if let Some((second, third)) = two_sum(total - n, &list[idx + 1..]) {
            return Some((*n, second, third));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_with_complement() {
        let result = two_sum(2020, &[1721, 979, 366, 299, 675, 1456]);
        assert_eq!(Some((1721, 299)), result);
    }

    #[test]
    fn two_sum_without_complement() {
        let result = two_sum(10, &[1721, 979, 366, 299, 675, 1456]);
        assert_eq!(None, result);
    }

    #[test]
    fn three_sum_with_complement() {
        let result = three_sum(2020, &[1721, 979, 366, 299, 675, 1456]);
        assert_eq!(Some((979, 366, 675)), result);
    }

    #[test]
    fn threetwo_sum_without_complement() {
        let result = three_sum(10, &[1721, 979, 366, 299, 675, 1456]);
        assert_eq!(None, result);
    }
}
