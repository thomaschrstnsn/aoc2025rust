use std::{fmt::Display, ops::RangeInclusive, str::FromStr};

use itertools::Itertools as _;
use thiserror::Error;

advent_of_code::solution!(2);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Expected {
    Seperator(char),
    Number,
}

impl Display for Expected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expected::Seperator(c) => write!(f, "seperator {}", c),
            Expected::Number => f.write_str("number"),
        }
    }
}

#[derive(Error, Debug)]
enum IdRangeParsingError {
    #[error("invalid shape of input, found {found}, expected: {expected}")]
    InvalidShape { found: String, expected: Expected },
}

struct IdRange(RangeInclusive<usize>);

impl AsRef<RangeInclusive<usize>> for IdRange {
    fn as_ref(&self) -> &RangeInclusive<usize> {
        &self.0
    }
}

impl FromStr for IdRange {
    type Err = IdRangeParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_single(s)
    }
}

impl IdRange {
    pub fn parse_single(input: &str) -> Result<Self, IdRangeParsingError> {
        let (first, second) = input
            .split_once('-')
            .ok_or(IdRangeParsingError::InvalidShape {
                found: input.into(),
                expected: Expected::Seperator('-'),
            })?;

        let first: usize = first
            .parse()
            .map_err(|_| IdRangeParsingError::InvalidShape {
                found: first.into(),
                expected: Expected::Number,
            })?;
        let second: usize = second
            .parse()
            .map_err(|_| IdRangeParsingError::InvalidShape {
                found: second.into(),
                expected: Expected::Number,
            })?;

        Ok(Self(first..=second))
    }

    pub fn part_one_invalid_ids(&self) -> impl Iterator<Item = usize> {
        self.0.clone().filter(|id| part_one_invalid_id(*id))
    }

    pub fn part_two_invalid_ids(&self) -> impl Iterator<Item = usize> {
        self.0.clone().filter(|id| part_two_invalid_id(*id))
    }
}

fn part_one_invalid_id(id: usize) -> bool {
    if id.ilog10().is_multiple_of(2) {
        return false;
    }
    let decimals = format!("{}", id);
    let (first, second) = decimals.split_at(decimals.len() / 2);

    first == second
}

fn part_two_invalid_id(id: usize) -> bool {
    let decimals = format!("{}", id);

    let length = decimals.len();

    let max_chunk_length = length / 2;
    for chunk_length in 1..=max_chunk_length {
        if length % chunk_length != 0 {
            continue;
        }
        let mut chunks = decimals.as_bytes().chunks(chunk_length);
        if chunks.all_equal() {
            return true;
        }
    }

    false
}

struct IdRanges(Vec<IdRange>);

impl AsRef<Vec<IdRange>> for IdRanges {
    fn as_ref(&self) -> &Vec<IdRange> {
        &self.0
    }
}

impl FromStr for IdRanges {
    type Err = IdRangeParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .split(',')
            .map(IdRange::parse_single)
            .collect::<Result<Vec<_>, IdRangeParsingError>>()?;

        Ok(Self(ranges))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let ranges: IdRanges = input.parse().ok()?;
    let sum = ranges
        .as_ref()
        .iter()
        .flat_map(|range| range.part_one_invalid_ids())
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let ranges: IdRanges = input.parse().ok()?;
    let sum = ranges
        .as_ref()
        .iter()
        .flat_map(|range| range.part_two_invalid_ids())
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 11-22 has two invalid IDs, 11 and 22.
    #[test]
    fn first_invalid_ids() {
        let range: IdRange = "11-22".parse().expect("can parse");

        assert_eq!(
            range.part_one_invalid_ids().collect::<Vec<_>>(),
            vec![11, 22]
        );
    }
    // 95-115 has one invalid ID, 99.
    #[test]
    fn second_invalid_ids() {
        let range: IdRange = "95-115".parse().expect("can parse");

        assert_eq!(range.part_one_invalid_ids().collect::<Vec<_>>(), vec![99]);
    }
    // 998-1012 has one invalid ID, 1010.
    // 1188511880-1188511890 has one invalid ID, 1188511885.
    // 222220-222224 has one invalid ID, 222222.
    // 1698522-1698528 contains no invalid IDs.
    // 446443-446449 has one invalid ID, 446446.
    // 38593856-38593862 has one invalid ID, 38593859.
    // The rest of the ranges contain no invalid IDs.

    #[test]
    fn parse_single() {
        let actual = IdRange::parse_single("25-67").expect("can parse");

        assert_eq!(actual.as_ref(), &(25..=67));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
