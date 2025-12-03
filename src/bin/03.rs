use std::str::FromStr;

use thiserror::Error;

advent_of_code::solution!(3);

struct Battery(u8);

impl AsRef<u8> for Battery {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}

#[derive(Debug, Error)]
enum BatteryParsingError {
    #[error("Invalid battery format: {found} but expected decimal digit")]
    InvalidBatteryFormat { found: char },
}

impl TryFrom<char> for Battery {
    type Error = BatteryParsingError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii_digit() {
            Ok(Battery(value.to_digit(10).unwrap() as u8))
        } else {
            Err(BatteryParsingError::InvalidBatteryFormat { found: value })
        }
    }
}

struct Bank(Vec<Battery>);

impl AsRef<Vec<Battery>> for Bank {
    fn as_ref(&self) -> &Vec<Battery> {
        &self.0
    }
}

impl Bank {
    pub fn max_two_combination(&self) -> u64 {
        let (next_index, first) = self.max_from(0, self.0.len() - 1);

        let (_, second) = self.max_from(next_index + 1, self.0.len());

        (first * 10) as u64 + (second as u64)
    }

    pub fn max_n_combinations(&self, n: usize) -> u64 {
        let mut start = 0;
        let mut end = self.0.len() - n + 1;
        let mut sum = 0;

        debug_assert!(start < end);

        for _ in 1..=n {
            let (next_start, val) = self.max_from(start, end);

            sum = sum * 10 + val as u64;

            start = next_start + 1;
            end += 1;
        }

        sum
    }

    fn max_from(&self, start: usize, end: usize) -> (usize, u8) {
        let mut max = 0;
        let mut max_index = 0;
        for index in start..end {
            let val = *self.0[index].as_ref();
            if val == 9 {
                return (index, val);
            }
            if val > max {
                max = val;
                max_index = index;
            }
        }

        (max_index, max)
    }
}

impl FromStr for Bank {
    type Err = BatteryParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries = s
            .trim()
            .chars()
            .map(Battery::try_from)
            .collect::<Result<Vec<_>, Self::Err>>()?;

        Ok(Self(batteries))
    }
}

struct Banks(Vec<Bank>);

impl FromStr for Banks {
    type Err = BatteryParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let banks = s
            .lines()
            .map(str::parse::<Bank>)
            .collect::<Result<Vec<_>, Self::Err>>()?;

        Ok(Self(banks))
    }
}

impl Banks {
    pub fn sum_of_max_two_combinations(&self) -> u64 {
        self.0.iter().map(|bank| bank.max_two_combination()).sum()
    }
    pub fn sum_of_max_n_combinations(&self, n: usize) -> u64 {
        self.0.iter().map(|bank| bank.max_n_combinations(n)).sum()
    }
}

impl AsRef<Vec<Bank>> for Banks {
    fn as_ref(&self) -> &Vec<Bank> {
        &self.0
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input: Banks = input.parse().expect("parses");

    Some(input.sum_of_max_two_combinations())
}

pub fn part_two(input: &str) -> Option<u64> {
    let input: Banks = input.parse().expect("parses");

    Some(input.sum_of_max_n_combinations(12))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
