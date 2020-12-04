// Right 1, down 2 getting off by 1
pub fn num_trees(bytes: &[u8], slope: (usize, usize)) -> u64 {
    let width = bytes
        .iter()
        .position(|c| *c == b'\n')
        .map(|p| p + 1)
        .unwrap_or_else(|| bytes.len());

    let mut count = 0;
    let mut pos = 0;
    let mut iter = 0;

    while pos < bytes.len() {
        if bytes[pos] == b'#' {
            count += 1
        }

        iter += 1;

        pos = (iter * slope.1) * width + ((iter * slope.0) % (width - 1))
    }

    count
}

pub fn part_1(input: &[u8]) -> u64 {
    num_trees(&input, (3, 1))
}

pub fn part_2(input: &[u8]) -> u64 {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|slope| num_trees(&input, *slope))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_data() {
        assert_eq!(0, num_trees("".as_bytes(), (3, 1)));
    }

    #[test]
    fn first_space() {
        assert_eq!(1, num_trees("#....".as_bytes(), (3, 1)));
        assert_eq!(0, num_trees(".....".as_bytes(), (3, 1)));
    }

    #[test]
    fn second_space() {
        assert_eq!(
            2,
            num_trees(
                "#...\n\
                 ...#"
                    .as_bytes(),
                (3, 1)
            )
        );
        assert_eq!(
            2,
            num_trees(
                "#.......\n\
                 ...#...."
                    .as_bytes(),
                (3, 1)
            )
        );
    }

    #[test]
    fn wrapping() {
        assert_eq!(
            3,
            num_trees(
                "#...\n\
                 ...#\n\
                 ..#."
                    .as_bytes(),
                (3, 1)
            )
        );
        assert_eq!(
            3,
            num_trees(
                "#......\n\
                 ...#...\n\
                 ......#"
                    .as_bytes(),
                (3, 1)
            )
        );
    }

    #[test]
    fn slope_with_two_down() {
        assert_eq!(
            2,
            num_trees(
                "#...\n\
                 ....\n\
                 ...#\n"
                    .as_bytes(),
                (3, 2)
            )
        )
    }
}
