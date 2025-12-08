use std::{ops::Deref, str::FromStr};

use itertools::Itertools;
use thiserror::Error;

advent_of_code::solution!(8);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Vec3 {
    x: usize,
    y: usize,
    z: usize,
}

impl Vec3 {
    fn dist(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

#[derive(Debug)]
struct Problem(Vec<Vec3>);

impl Deref for Problem {
    type Target = Vec<Vec3>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Error)]
enum Vec3ParsingError {
    #[error("nan nan nan")]
    NotAnActualNumber(#[from] std::num::ParseIntError),

    #[error("invalid structure, expected three numbers with two commas between (dos commas!)")]
    NotDosCommas,
}

impl FromStr for Vec3 {
    type Err = Vec3ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().ok_or(Vec3ParsingError::NotDosCommas)?;
        let y = parts.next().ok_or(Vec3ParsingError::NotDosCommas)?;
        let z = parts.next().ok_or(Vec3ParsingError::NotDosCommas)?;
        if parts.next().is_some() {
            return Err(Vec3ParsingError::NotDosCommas);
        }

        let x = x.parse()?;
        let y = y.parse()?;
        let z = z.parse()?;

        Ok(Self { x, y, z })
    }
}

impl FromStr for Problem {
    type Err = Vec3ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vecs = s
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(vecs))
    }
}

fn find_n_closest(input: &[Vec3], n: usize) -> usize {
    let mut members = input.iter().map(|_| 1usize).collect::<Vec<_>>();
    let mut circuits = input.iter().enumerate().collect::<Vec<_>>();

    let input = input
        .iter()
        .tuple_combinations()
        .k_smallest_by_key(n, |(a, b)| a.dist(b));

    for next in input {
        let fi = circuits.iter().position(|(_, c)| *c == next.0).unwrap();
        let si = circuits.iter().position(|(_, c)| *c == next.1).unwrap();

        // merge circuits
        let first_circuit_id = circuits[fi].0;
        let second_circuit_id = circuits[si].0;

        if first_circuit_id == second_circuit_id {
            continue; // already connected
        }

        for (cid, _) in circuits.iter_mut() {
            if *cid == second_circuit_id {
                *cid = first_circuit_id;
            }
        }
        members[first_circuit_id] += members[second_circuit_id];
        members[second_circuit_id] = 0;
    }

    members.into_iter().k_largest(3).product()
}

pub fn part_one_parameterized(input: &str, n: usize) -> Option<usize> {
    let problem: Problem = input.parse().expect("parses");

    let vecs: &Vec<Vec3> = &problem;

    let result = find_n_closest(vecs, n);

    Some(result)
}

pub fn part_one_example(input: &str) -> Option<usize> {
    part_one_parameterized(input, 10)
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_parameterized(input, 1000)
}

fn find_last_closing_connection(input: &[Vec3]) -> usize {
    let original_len = input.len();
    let mut members = input.iter().map(|_| 1usize).collect::<Vec<_>>();
    let mut circuits = input.iter().enumerate().collect::<Vec<_>>();

    let input = input
        .iter()
        .tuple_combinations()
        .sorted_by_key(|(a, b)| a.dist(b));

    for next in input {
        let fi = circuits.iter().position(|(_, c)| *c == next.0).unwrap();
        let si = circuits.iter().position(|(_, c)| *c == next.1).unwrap();

        // merge circuits
        let first_circuit_id = circuits[fi].0;
        let second_circuit_id = circuits[si].0;
        if first_circuit_id == second_circuit_id {
            continue; // already connected
        }

        for (cid, _) in circuits.iter_mut() {
            if *cid == second_circuit_id {
                *cid = first_circuit_id;
            }
        }

        members[first_circuit_id] += members[second_circuit_id];
        members[second_circuit_id] = 0;

        if members[first_circuit_id] == original_len {
            let fix = next.0.x;
            let six = next.1.x;

            return fix * six;
        }
    }

    0
}

pub fn part_two(input: &str) -> Option<usize> {
    let problem: Problem = input.parse().expect("parses");

    let vecs: &Vec<Vec3> = &problem;

    let result = find_last_closing_connection(vecs);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_example(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
