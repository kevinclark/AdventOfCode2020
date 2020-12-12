use std::convert::TryInto;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Split {
    Front,
    Back,
    Left,
    Right,
}

pub fn parse(description: &[u8; 10]) -> [Split; 10] {
    use Split::*;

    let mut results: [Split; 10] = [Front; 10];

    for (i, c) in description.iter().enumerate() {
        results[i] = match c {
            b'F' => Front,
            b'B' => Back,
            b'L' => Left,
            b'R' => Right,
            _ => panic!("Unknown split: {}", c),
        }
    }

    results
}

fn locate_seat(path: [Split; 10]) -> (u8, u8) {
    use Split::*;

    let mut step = 64;
    let mut lower = 0;
    let mut upper = 127u8;

    for p in path[..6].iter() {
        match p {
            Front => upper -= step,
            Back => lower += step,
            _ => panic!("Didn't expect {:?} in row context", p),
        }

        step = step >> 1;
    }

    let row = match path[6] {
        Front => lower,
        Back => upper,
        _ => panic!("Didn't expect {:?} in row context", path[6]),
    };

    let mut step = 4;
    let mut lower = 0;
    let mut upper = 7u8;

    for p in path[7..9].iter() {
        match p {
            Right => lower += step,
            Left => upper -= step,
            _ => panic!("Didn't expect {:?} in col context", p),
        }

        step = step >> 1;
    }

    let col = match path[9] {
        Right => upper,
        Left => lower,
        _ => panic!("Didn't expect {:?} in col context", path[9]),
    };

    (row, col)
}

pub fn seat_id(description: &[u8]) -> u32 {
    let (row, col) = locate_seat(parse(description.try_into().unwrap()));
    (row as u32) * 8u32 + (col as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        use Split::*;

        assert_eq!(
            [
                Front, Back, Front, Back, Back, Front, Front, Right, Left,
                Right
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
        assert_eq!(567u32, seat_id("BFFFBBFRRR".as_bytes().try_into().unwrap()))
    }
}
