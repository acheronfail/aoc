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
            Some(hcl) => {
                let hcl = hcl.chars().collect::<Vec<char>>();
                if hcl[0] != '#'
                    || hcl.len() != 7
                    || hcl.iter().all(|c| match c {
                        '0'..='9' => true,
                        'a'..='f' => true,
                        _ => false,
                    })
                {
                    return false;
                }
            }
            None => return false,
        }

        match self.hgt.as_ref() {
            Some(s) => {
                let chars = s.chars().collect::<Vec<char>>();
                let suffix = chars[chars.len() - 2..].iter().collect::<String>();
                let (min, max) = match suffix.as_str() {
                    "cm" => (150, 193),
                    "in" => (59, 76),
                    _ => return false,
                };

                let prefix = chars[..chars.len() - 2].iter().collect::<String>();
                if !check_number(Some(&prefix), min, max) {
                    return false;
                }
            }
            None => return false,
        }

        match self.ecl.as_ref().map(|s| s.as_str()) {
            Some(s) => match s {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => return false,
            },
            None => return false,
        };

        match self.pid.as_ref() {
            Some(s) => {
                if s.len() != 9 || s.parse::<usize>().is_err() {
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
        let mut passport = Passport::default();
        let parts = s.split_ascii_whitespace().collect::<Vec<&str>>();
        for part in parts {
            let split = part.split(':').collect::<Vec<&str>>();
            let id = split[0];
            let value = Some(split[1].to_string());
            match id {
                "byr" => passport.byr = value,
                "iyr" => passport.iyr = value,
                "eyr" => passport.eyr = value,
                "hgt" => passport.hgt = value,
                "hcl" => passport.hcl = value,
                "ecl" => passport.ecl = value,
                "pid" => passport.pid = value,
                "cid" => passport.cid = value,
                _ => {}
            }
        }

        passport
    }
}

fn check_number(x: Option<&String>, min: isize, max: isize) -> bool {
    if x.is_none() {
        return false;
    }

    let y = x.as_ref().unwrap().parse::<isize>().unwrap();
    min <= y && y <= max
}

fn main() {
    let input = include_str!("./4.txt");

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
