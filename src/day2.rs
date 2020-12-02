use std::str;

fn parse<'a>(line: &'a str) -> (usize, usize, &'a str, &'a str) {
    let mut parts = line.split(&['-', ' ', ':'][..]);

    let start = parts
        .by_ref()
        .take(1)
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let end = parts
        .by_ref()
        .take(1)
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let letter = parts.by_ref().take(1).next().unwrap();
    let pass = parts.by_ref().skip(1).take(1).next().unwrap();

    (start, end, letter, pass)
}

pub fn number_of_valid_part_2_passwords(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (start, end, letter, pass) = parse(line);

            let first = pass.as_bytes()[start - 1];
            let second = pass.as_bytes()[end - start];

            let first_match = first == letter.as_bytes()[0];
            let second_match = second == letter.as_bytes()[0];

            first_match ^ second_match
        })
        .count()
}

pub fn number_of_valid_part_1_passwords(input: &str) -> usize {
    // N-N L: pass
    input
        .lines()
        .filter(|line| {
            let (start, end, letter, pass) = parse(line);
            let range = start..=end;
            let occurences = pass
                .chars()
                .filter(|l| *l == letter.chars().nth(0).unwrap())
                .count();

            range.contains(&occurences)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "1-3 a: abcde";
        let (start, end, letter, pass) = parse(&input);
        assert_eq!(1, start);
        assert_eq!(3, end);
        assert_eq!("a", letter);
        assert_eq!("abcde", pass);
    }

    #[test]
    fn part_one() {
        assert_eq!(1, number_of_valid_part_1_passwords(&"1-3 a: abcde"));
        assert_eq!(0, number_of_valid_part_1_passwords(&"1-3 x: abcde"));
    }

    #[test]
    fn part_two() {
        assert_eq!(1, number_of_valid_part_2_passwords(&"1-3 a: abcde"));
        assert_eq!(1, number_of_valid_part_2_passwords(&"1-3 a: cbade"));
        assert_eq!(0, number_of_valid_part_2_passwords(&"1-3 x: cabde"));
    }
}
