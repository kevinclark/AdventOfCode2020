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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_complement() {
        let result = two_sum(2020, &[1721, 979, 366, 299, 675, 1456]);
        assert_eq!(Some((1721, 299)), result);
    }

    #[test]
    fn without_complement() {
        let result = two_sum(10, &[1721, 979, 366, 299, 675, 1456]);
        assert_eq!(None, result);
    }
}
