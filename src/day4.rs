// Masks
const BYR: u8 = 0x80; // byr (Birth Year)
const IYR: u8 = 0x40; // iyr (Issue Year)
const EYR: u8 = 0x20; // eyr (Expiration Year)
const HGT: u8 = 0x10; // hgt (Height)
const HCL: u8 = 0x08; // hcl (Hair Color)
const ECL: u8 = 0x04; // ecl (Eye Color)
const PID: u8 = 0x02; // pid (Passport ID)
const CID: u8 = 0x01; // cid (Country ID)

fn is_valid(passport_declaration: &str) -> bool {
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

pub fn valid_passports(input: &str) -> usize {
    input.split("\n\n").filter(|line| is_valid(line)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_with_cid() {
        let passport = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                        byr:1937 iyr:2017 cid:147 hgt:183cm\n\n";
        assert!(is_valid(passport));
    }

    #[test]
    fn valid_without_cid() {
        let passport = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                        byr:1937 iyr:2017 hgt:183cm\n\n";
        assert!(is_valid(passport));
    }

    #[test]
    fn invalid_without_ecl() {
        let passport = "pid:860033327 eyr:2020 hcl:#fffffd\n\
                        byr:1937 iyr:2017 hgt:183cm\n\n";
        assert!(!is_valid(passport));
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

        assert_eq!(2, valid_passports(passports));
    }
}
