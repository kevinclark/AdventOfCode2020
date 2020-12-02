use std::str;

pub fn number_of_valid_passwords(input: &str) -> usize {
    // N-N L: pass
    input
        .lines()
        .filter(|line| {
            let mut chars = line.chars();

            let start = chars
                .by_ref()
                .take_while(|c| *c != '-')
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            let end = chars
                .by_ref()
                .take_while(|c| *c != ' ')
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            let range = start..=end;

            let letter = chars.next().unwrap();

            let pass: String = chars.skip(2).collect();

            let occurences = pass.chars().filter(|l| *l == letter).count();

            range.contains(&occurences)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_match() {
        assert_eq!(1, number_of_valid_passwords(&"1-3 a: abcde"));
    }

    #[test]
    fn no_match() {
        assert_eq!(0, number_of_valid_passwords(&"1-3 x: abcde"));
    }
}
