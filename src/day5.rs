use std::convert::TryInto;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Split {
    Lower,
    Upper,
}

pub fn parse(description: &[u8; 10]) -> [Split; 10] {
    use Split::*;

    let mut results: [Split; 10] = [Lower; 10];

    for (i, c) in description.iter().enumerate() {
        results[i] = match c {
            b'F' | b'L' => Lower,
            b'B' | b'R' => Upper,
            _ => panic!("Unknown split: {}", c),
        }
    }

    results
}

fn traverse(path: &[Split]) -> usize {
    use Split::*;

    let mut step = 1usize << (path.len() - 1);
    let mut lower = 0usize;
    let mut upper = (step << 1) - 1;

    for p in path[..path.len() - 1].iter() {
        match p {
            Lower => upper = upper.checked_sub(step).unwrap(),
            Upper => lower += step,
        }

        step = step >> 1;
    }

    match path[path.len() - 1] {
        Lower => lower,
        Upper => upper,
    }
}

pub fn locate_seat(path: [Split; 10]) -> (usize, usize) {
    let row = traverse(&path[..7]);
    let col = traverse(&path[7..]);

    (row, col)
}

pub fn seat_id(description: &[u8]) -> usize {
    let (row, col) = locate_seat(parse(description.try_into().unwrap()));
    row * 8usize + col
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        use Split::*;

        assert_eq!(
            [
                Lower, Upper, Lower, Upper, Upper, Lower, Lower, Upper, Lower,
                Upper
            ],
            parse("FBFBBFFRLR".as_bytes().try_into().unwrap())
        )
    }

    #[test]
    fn locating() {
        assert_eq!(
            (44, 5),
            locate_seat(parse("FBFBBFFRLR".as_bytes().try_into().unwrap()))
        )
    }

    #[test]
    fn seat_ids() {
        assert_eq!(567, seat_id("BFFFBBFRRR".as_bytes().try_into().unwrap()))
    }
}
