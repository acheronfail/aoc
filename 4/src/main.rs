#[derive(Default)]
struct Passport {
    /// Birth Year
    pub byr: Option<String>,
    /// Issue Year
    pub iyr: Option<String>,
    /// Expiration Year
    pub eyr: Option<String>,
    /// Height
    pub hgt: Option<String>,
    /// Hair Color
    pub hcl: Option<String>,
    /// Eye Color
    pub ecl: Option<String>,
    /// Passport ID
    pub pid: Option<String>,
    /// Country ID
    pub cid: Option<String>,
}

impl Passport {
    pub fn part_1_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    pub fn part_2_valid(&self) -> bool {
        match self.hcl.as_ref() {
            Some(s) => {
                let hcl = s.chars().collect::<Vec<char>>();
                let hcl_chars = [
                    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
                ];
                if hcl[0] != '#' || hcl.len() != 7 || hcl.iter().all(|c| hcl_chars.contains(c)) {
                    return false;
                }
            }
            None => return false,
        }

        match self.hgt.as_ref() {
            Some(s) => {
                let chars = s.chars().collect::<Vec<char>>();
                let suffix = chars[chars.len() - 2..].iter().collect::<String>();
                let prefix = chars[..chars.len() - 2].iter().collect::<String>();

                match suffix.as_str() {
                    "cm" => {
                        if !check_number(Some(&prefix), 150, 193) {
                            return false;
                        }
                    }

                    "in" => {
                        if !check_number(Some(&prefix), 59, 76) {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
            None => {
                return false;
            }
        }

        match self.ecl.as_ref().map(|s| s.as_str()) {
            Some("amb") | Some("blu") | Some("brn") | Some("gry") | Some("grn") | Some("hzl")
            | Some("oth") => true,
            _ => return false,
        };

        match self.pid.as_ref() {
            Some(s) => {
                if s.len() != 9 {
                    return false;
                }

                if s.parse::<usize>().is_err() {
                    return false;
                }
            }
            None => return false,
        }

        check_number(self.byr.as_ref(), 1920, 2002)
            && check_number(self.iyr.as_ref(), 2010, 2020)
            && check_number(self.eyr.as_ref(), 2020, 2030)
    }
}

impl From<&str> for Passport {
    fn from(s: &str) -> Self {
        let mut p = Passport::default();

        let parts = s.split_ascii_whitespace().collect::<Vec<&str>>();
        for part in parts {
            let split = part.split(':').collect::<Vec<&str>>();
            let id = split[0];
            let value = split[1];
            match id {
                "byr" => p.byr = Some(value.to_string()),
                "iyr" => p.iyr = Some(value.to_string()),
                "eyr" => p.eyr = Some(value.to_string()),
                "hgt" => p.hgt = Some(value.to_string()),
                "hcl" => p.hcl = Some(value.to_string()),
                "ecl" => p.ecl = Some(value.to_string()),
                "pid" => p.pid = Some(value.to_string()),
                "cid" => p.cid = Some(value.to_string()),
                _ => {}
            }
        }

        p
    }
}

fn check_number(x: Option<&String>, min: isize, max: isize) -> bool {
    if x.is_none() {
        return false;
    }
    let y = x.as_ref().unwrap().parse::<isize>().unwrap();
    if y < min || y > max {
        return false;
    }

    true
}

fn main() {
    let input = include_str!("./input.txt");

    let mut passports = vec![];
    let mut passport_str = String::new();
    for line in input.lines() {
        if line.len() == 0 {
            passports.push(passport_str.clone());
            passport_str = String::new();
        }

        passport_str.push('\n');
        passport_str.push_str(line);
    }

    let passports = passports
        .iter()
        .map(|s| Passport::from(s.as_str()))
        .collect::<Vec<Passport>>();

    println!(
        "Part 1: {}",
        passports.iter().filter(|p| p.part_1_valid()).count()
    );
    println!(
        "Part 2: {}",
        passports.iter().filter(|p| p.part_2_valid()).count()
    );
}
