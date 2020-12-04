use std::ops::RangeInclusive;

struct PasswordWithPolicy {
    range: RangeInclusive<usize>,
    character: char,
    password: String,
}

impl PasswordWithPolicy {
    pub fn is_valid_part_1(&self) -> bool {
        self.range.contains(
            &self
                .password
                .chars()
                .filter(|ch| *ch == self.character)
                .count(),
        )
    }

    pub fn is_valid_part_2(&self) -> bool {
        // convert to zero indexed
        let idx_1 = self.range.start() - 1;
        let idx_2 = self.range.end() - 1;
        let chars = self.password.chars().collect::<Vec<char>>();

        if idx_1 >= chars.len() {
            return false;
        }

        let match_1 = chars[idx_1] == self.character;
        if idx_2 >= chars.len() {
            return match_1;
        }

        let match_2 = chars[idx_2] == self.character;
        match_1 != match_2
    }
}

impl From<&str> for PasswordWithPolicy {
    fn from(s: &str) -> Self {
        let policy_and_password = s.split(':').collect::<Vec<&str>>();
        let policy_parts = policy_and_password[0].split(' ').collect::<Vec<&str>>();
        let policy_range = policy_parts[0].split('-').collect::<Vec<&str>>();
        let start = policy_range[0].parse::<usize>().unwrap();
        let end = policy_range[1].parse::<usize>().unwrap();
        PasswordWithPolicy {
            password: policy_and_password[1].trim().to_string(),
            character: policy_parts[1].chars().last().unwrap(),
            range: start..=end,
        }
    }
}

fn main() {
    let mut part_1_valid_count = 0;
    let mut part_2_valid_count = 0;
    for line in include_str!("./2.txt").lines() {
        let pwp = PasswordWithPolicy::from(line);
        if pwp.is_valid_part_1() {
            part_1_valid_count += 1;
        }
        if pwp.is_valid_part_2() {
            part_2_valid_count += 1;
        }
    }

    println!("Part 1: {}", part_1_valid_count);
    println!("Part 2: {}", part_2_valid_count);
}
