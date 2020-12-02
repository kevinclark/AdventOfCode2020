use std::str;

fn parse<'a>(line: &'a str) -> (usize, usize, u8, &'a [u8]) {
    let mut parts = line
        .as_bytes()
        .split(|c| *c == b'-' || *c == b' ' || *c == b':'); // .split(&[b'-', b' ', b':'][..]);

    let start = parts
        .by_ref()
        .take(1)
        .next()
        .unwrap()
        .iter()
        .fold(0, |acc, i| acc * 10 + (*i - b'0') as usize);
    let end = parts
        .by_ref()
        .take(1)
        .next()
        .unwrap()
        .iter()
        .fold(0, |acc, i| acc * 10 + (*i - b'0') as usize);
    let letter = parts.by_ref().take(1).next().unwrap()[0];
    let pass = parts.by_ref().skip(1).take(1).next().unwrap();

    (start, end, letter, pass)
}

pub fn number_of_valid_part_2_passwords(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (start, end, letter, pass) = parse(line);

            let first = pass[start - 1];
            let second = pass[end - start];

            let first_match = first == letter;
            let second_match = second == letter;

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
            let needle = letter;
            let occurences = pass.iter().filter(|c| *c == &needle).count();

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
        assert_eq!(b'a', letter);
        assert_eq!("abcde".as_bytes(), pass);
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
