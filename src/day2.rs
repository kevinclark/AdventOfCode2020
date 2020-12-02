use std::str;

fn parse<'a>(
    line: &'a str,
) -> (usize, usize, char, impl Iterator<Item = char> + 'a) {
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

    let letter = chars.next().unwrap();

    (start, end, letter, chars.skip(2))
}

pub fn number_of_valid_part_2_passwords(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (start, end, letter, mut pass) = parse(line);

            let first = pass.by_ref().nth(start - 1).unwrap();
            let second = pass.by_ref().nth(end - start - 1).unwrap();

            let first_match = first == letter;
            let second_match = second == letter;

            (first_match || second_match) && !(first_match && second_match)
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
            let occurences = pass.filter(|l| *l == letter).count();

            range.contains(&occurences)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

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
