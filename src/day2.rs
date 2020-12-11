fn parse<'a>(line: &'a [u8]) -> (usize, usize, u8, &'a [u8]) {
    let mut parts = line.split(|c| *c == b'-' || *c == b' ' || *c == b':');

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

pub fn number_of_valid_part_2_passwords(input: &[u8]) -> usize {
    input
        .split(|c| *c == b'\n')
        .filter(|line| {
            if line.is_empty() {
                false
            } else {
                let (start, end, letter, pass) = parse(&line);

                let first = pass[start - 1];
                let second = pass[end - 1];

                let first_match = first == letter;
                let second_match = second == letter;

                first_match ^ second_match
            }
        })
        .count()
}

pub fn number_of_valid_part_1_passwords(input: &[u8]) -> usize {
    // N-N L: pass
    input
        .split(|c| *c == b'\n')
        .filter(|line| {
            if line.is_empty() {
                false
            } else {
                let (start, end, letter, pass) = parse(&line);

                let range = start..=end;
                let occurences = pass.iter().filter(|c| *c == &letter).count();

                range.contains(&occurences)
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "1-3 a: abcde";
        let (start, end, letter, pass) = parse(&input.as_bytes());
        assert_eq!(1, start);
        assert_eq!(3, end);
        assert_eq!(b'a', letter);
        assert_eq!("abcde".as_bytes(), pass);
    }

    #[test]
    fn part_one() {
        assert_eq!(
            1,
            number_of_valid_part_1_passwords(&"1-3 a: abcde".as_bytes())
        );
        assert_eq!(
            0,
            number_of_valid_part_1_passwords(&"1-3 x: abcde".as_bytes())
        );
        assert_eq!(
            2,
            number_of_valid_part_1_passwords(
                &"1-3 a: abcde\n1-3 b: abcde\n5-10 a: abcde\n".as_bytes()
            )
        );

        assert_eq!(
            2,
            number_of_valid_part_1_passwords(
                &"1-3 a: abcde\n\
                                                         1-3 b: cdefg\n\
                                                         2-9 c: ccccccccc\n"
                    .as_bytes()
            )
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            1,
            number_of_valid_part_2_passwords(&"1-3 a: abcde".as_bytes())
        );
        assert_eq!(
            1,
            number_of_valid_part_2_passwords(&"1-3 a: cbade".as_bytes())
        );
        assert_eq!(
            0,
            number_of_valid_part_2_passwords(&"1-3 x: cabde".as_bytes())
        );

        assert_eq!(
            2,
            number_of_valid_part_2_passwords(
                &"1-3 a: abcde\n\
                                                         1-3 b: cdefg\n\
                                                         2-4 b: cdebg\n\
                                                         2-9 c: ccccccccc\n"
                    .as_bytes()
            )
        );
    }
}
