use std::convert::TryInto;
use std::ops::RangeInclusive;

// Masks
const BYR: u8 = 0x80; // byr (Birth Year)
const IYR: u8 = 0x40; // iyr (Issue Year)
const EYR: u8 = 0x20; // eyr (Expiration Year)
const HGT: u8 = 0x10; // hgt (Height)
const HCL: u8 = 0x08; // hcl (Hair Color)
const ECL: u8 = 0x04; // ecl (Eye Color)
const PID: u8 = 0x02; // pid (Passport ID)
const CID: u8 = 0x01; // cid (Country ID)

fn has_valid_fields(passport_declaration: &str) -> bool {
    passport_declaration
        .split(&['\n', ' ', ':'][..])
        .fold(0u8, |acc, part| match part {
            "byr" => acc | BYR,
            "iyr" => acc | IYR,
            "eyr" => acc | EYR,
            "hgt" => acc | HGT,
            "hcl" => acc | HCL,
            "ecl" => acc | ECL,
            "pid" => acc | PID,
            "cid" => acc | CID,
            _ => acc,
        })
        >= 0xfe
}

pub fn number_of_passports_with_valid_fields(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|line| has_valid_fields(line))
        .count()
}

enum PassportState {
    Valid { mask: u8 },
    Invalid,
}

fn parse_usize(bytes: &[u8]) -> usize {
    bytes
        .iter()
        .fold(0, |acc, i| acc * 10 + (*i - b'0') as usize)
}

fn validate_year(
    value: &[u8],
    range: RangeInclusive<usize>,
    current_mask: u8,
    bit: u8,
) -> PassportState {
    if value.len() == 4 && range.contains(&parse_usize(&value)) {
        PassportState::Valid {
            mask: current_mask | bit,
        }
    } else {
        PassportState::Invalid
    }
}

fn validate_height(value: &[u8], current_mask: u8) -> PassportState {
    use PassportState::*;
    match value[value.len() - 2..] {
        [b'c', b'm'] => {
            let parsed = parse_usize(&value[..(value.len() - 2)]);
            if parsed >= 150 && parsed <= 193 {
                Valid {
                    mask: current_mask | HGT,
                }
            } else {
                Invalid
            }
        }
        [b'i', b'n'] => {
            let parsed = parse_usize(&value[..(value.len() - 2)]);
            if parsed >= 59 && parsed <= 76 {
                Valid {
                    mask: current_mask | HGT,
                }
            } else {
                Invalid
            }
        }
        _ => Invalid,
    }
}

fn validate_hair_color(value: &[u8], current_mask: u8) -> PassportState {
    if value[0] == b'#'
        && value[1..].len() == 6
        && value[1..]
            .iter()
            .all(|c| (*c >= b'0' && *c <= b'9') || (*c >= b'a' && *c <= b'f'))
    {
        PassportState::Valid {
            mask: current_mask | HCL,
        }
    } else {
        PassportState::Invalid
    }
}

fn validate_eyecolor(value: &[u8], current_mask: u8) -> PassportState {
    use PassportState::*;

    let options = [
        [b'a', b'm', b'b'],
        [b'b', b'l', b'u'],
        [b'b', b'r', b'n'],
        [b'g', b'r', b'y'],
        [b'g', b'r', b'n'],
        [b'h', b'z', b'l'],
        [b'o', b't', b'h'],
    ];

    value
        .try_into()
        .map(|v| {
            if options.contains(v) {
                Valid {
                    mask: current_mask | ECL,
                }
            } else {
                Invalid
            }
        })
        .unwrap_or(Invalid)
}

fn has_valid_fields_and_values(passport_declaration: &str) -> bool {
    use PassportState::*;

    let state = passport_declaration
        .split(&['\n', ' '][..])
        .filter(|s| !s.is_empty())
        .map(|s| s.as_bytes().split_at(3))
        .fold(Valid { mask: 0 }, |state, (field, value)| {
            let value = &value[1..]; // Skip the colon

            match state {
                Invalid => Invalid,

                Valid { mask } => match field {
                    [b'b', b'y', b'r'] => {
                        validate_year(value, 1920..=2002, mask, BYR)
                    }
                    [b'i', b'y', b'r'] => {
                        validate_year(value, 2010..=2020, mask, IYR)
                    }

                    [b'e', b'y', b'r'] => {
                        validate_year(value, 2020..=2030, mask, EYR)
                    }

                    [b'h', b'g', b't'] => validate_height(value, mask),

                    [b'h', b'c', b'l'] => validate_hair_color(value, mask),

                    [b'e', b'c', b'l'] => validate_eyecolor(value, mask),

                    [b'p', b'i', b'd'] => {
                        if value.len() == 9
                            && value.iter().all(|c| *c >= b'0' && *c <= b'9')
                        {
                            Valid { mask: mask | PID }
                        } else {
                            Invalid
                        }
                    }

                    [b'c', b'i', b'd'] => Valid { mask: mask | CID },
                    _ => state,
                },
            }
        });

    match state {
        Valid { mask } => mask >= 0xfe,
        _ => false,
    }
}

pub fn number_of_passports_with_valid_fields_and_values(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|line| has_valid_fields_and_values(line))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_with_cid() {
        let passport = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                        byr:1937 iyr:2017 cid:147 hgt:183cm\n\n";
        assert!(has_valid_fields(passport));
    }

    #[test]
    fn valid_without_cid() {
        let passport = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                        byr:1937 iyr:2017 hgt:183cm\n\n";
        assert!(has_valid_fields(passport));
    }

    #[test]
    fn invalid_without_ecl() {
        let passport = "pid:860033327 eyr:2020 hcl:#fffffd\n\
                        byr:1937 iyr:2017 hgt:183cm\n\n";
        assert!(!has_valid_fields(passport));
    }

    #[test]
    fn multiple_passports() {
        let passports = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        assert_eq!(2, number_of_passports_with_valid_fields(passports));
    }

    #[test]
    fn invalid_byr() {
        let passport =
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:2003\n\
                        hcl:#623a2f\n\n";
        assert!(!has_valid_fields_and_values(passport));
    }

    #[test]
    fn invalid_iyr() {
        let passport =
            "pid:087499704 hgt:74in ecl:grn iyr:2000 eyr:2030 byr:2002\n\
                        hcl:#623a2f\n\n";
        assert!(!has_valid_fields_and_values(passport));
    }

    #[test]
    fn invalid_eyr() {
        let passport =
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2010 byr:2002\n\
                        hcl:#623a2f\n\n";
        assert!(!has_valid_fields_and_values(passport));
    }

    #[test]
    fn invalid_hgt() {
        let passport =
            "pid:087499704 hgt:190in ecl:grn iyr:2012 eyr:2030 byr:2002\n\
                        hcl:#623a2f\n\n";
        assert!(!has_valid_fields_and_values(passport));
    }

    #[test]
    fn invalid_hcl() {
        let passport =
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:2002\n\
                        hcl:623a2f\n\n";
        assert!(!has_valid_fields_and_values(passport));

        let passport =
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:2002\n\
                        hcl:#623a2\n\n";
        assert!(!has_valid_fields_and_values(passport));
    }

    #[test]
    fn invalid_pid() {
        let passport =
            "pid:08749704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:2002\n\
                        hcl:#623a2f\n\n";
        assert!(!has_valid_fields_and_values(passport));
    }
}
