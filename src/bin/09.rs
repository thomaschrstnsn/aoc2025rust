use std::{num::ParseIntError, ops::Deref, str::FromStr};

use itertools::Itertools;
use thiserror::Error;

advent_of_code::solution!(9);

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn area_of_box(&self, other: &Self) -> usize {
        let width = self.x.abs_diff(other.x) + 1;
        let height = self.y.abs_diff(other.y) + 1;

        let res = width * height;

        res
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Invalid format, expected two numbers seperated by comma, found: {found}")]
    InvalidFormat { found: String },
    #[error("Invalid number: {0}")]
    InvalidNumber(#[from] ParseIntError),
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParseError::InvalidFormat {
            found: s.to_string(),
        })?;

        let x = x.parse()?;
        let y = y.parse()?;

        Ok(Self { x, y })
    }
}

struct Points(Vec<Point>);

impl Deref for Points {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Points {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Point>, ParseError>>()?;

        Ok(Self(points))
    }
}

impl Points {
    fn area_of_largest_box(&self) -> usize {
        self.iter()
            .tuple_combinations()
            .map(|(a, b)| a.area_of_box(b))
            .max()
            .unwrap()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let points: Points = input.parse().expect("can parse");

    points.area_of_largest_box().into()
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
