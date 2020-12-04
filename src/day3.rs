pub fn num_trees(bytes: &[u8]) -> u64 {
    let width = bytes
        .iter()
        .position(|c| *c == b'\n')
        .map(|p| p + 1)
        .unwrap_or_else(|| bytes.len());

    let mut row = 0;
    let mut col = 0;
    let mut count = 0;
    let mut pos = 0;

    while pos < bytes.len() {
        if bytes[pos] == b'#' {
            count += 1
        }

        row += 1;
        col += 3;

        pos = row * width + (col % (width - 1))
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_data() {
        assert_eq!(0, num_trees("".as_bytes()));
    }

    #[test]
    fn first_space() {
        assert_eq!(1, num_trees("#....".as_bytes()));
        assert_eq!(0, num_trees(".....".as_bytes()));
    }

    #[test]
    fn second_space() {
        assert_eq!(
            2,
            num_trees(
                "#...\n\
                 ...#"
                    .as_bytes()
            )
        );
        assert_eq!(
            2,
            num_trees(
                "#.......\n\
                 ...#...."
                    .as_bytes()
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
                    .as_bytes()
            )
        );
        assert_eq!(
            3,
            num_trees(
                "#......\n\
                 ...#...\n\
                 ......#"
                    .as_bytes()
            )
        );
    }
}
