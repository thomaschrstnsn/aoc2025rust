use std::str::Lines;

use thiserror::Error;

advent_of_code::solution!(1);

#[derive(Debug)]
enum Operation {
    Left(usize),
    Right(usize),
}

struct Operations(Vec<Operation>);

impl AsRef<[Operation]> for Operations {
    fn as_ref(&self) -> &[Operation] {
        &self.0
    }
}

#[derive(Error, Debug)]
enum OperationParsingError {
    #[error("invalid shape of input, found {found}")]
    InvalidShape { found: String },

    #[error("invalid offset, found: {found} expected digit")]
    InvalidOffset { found: String },
}

impl TryFrom<Lines<'_>> for Operations {
    type Error = OperationParsingError;

    fn try_from(value: Lines) -> Result<Self, Self::Error> {
        let vec: Vec<Operation> = value
            .map(|line| line.try_into())
            .collect::<Result<Vec<_>, OperationParsingError>>()?;

        Ok(Self(vec))
    }
}

impl TryFrom<&str> for Operation {
    type Error = OperationParsingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (dir, num) = value.split_at(1);
        if dir.len() != 1 || num.is_empty() {
            return Err(OperationParsingError::InvalidShape {
                found: value.into(),
            });
        }

        let num: usize = num
            .parse()
            .map_err(|_| OperationParsingError::InvalidOffset { found: num.into() })?;

        let res = match dir.chars().next().unwrap() {
            'L' => Operation::Left(num),
            'R' => Operation::Right(num),
            _ => {
                return Err(OperationParsingError::InvalidShape {
                    found: value.to_string(),
                });
            }
        };
        Ok(res)
    }
}

#[derive(Debug)]
struct LockState(usize);

impl Default for LockState {
    fn default() -> Self {
        LockState(50)
    }
}

impl LockState {
    fn apply(&mut self, op: &Operation) -> usize {
        let offset = match op {
            Operation::Left(offset) => offset,
            Operation::Right(offset) => offset,
        };

        let mut zeros: usize = offset / 100;
        let offset = offset % 100;
        let mut next = match op {
            Operation::Left(_) => (self.0 as isize) - (offset as isize),
            Operation::Right(_) => (self.0 as isize) + (offset as isize),
        };

        if next <= 0 && self.0 != 0 {
            zeros += 1;
        }
        if next >= 100 {
            zeros += 1;
        }
        if next < 0 {
            next += 100;
        }
        assert!(next >= 0);

        self.0 = (next % 100) as usize;

        zeros
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl From<&LockState> for usize {
    fn from(value: &LockState) -> Self {
        value.0
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let ops: Operations = input.lines().try_into().unwrap();

    let mut zeros = 0;
    let mut state = LockState::default();

    for op in ops.as_ref() {
        state.apply(op);
        if state.is_zero() {
            zeros += 1;
        }
    }

    Some(zeros)
}

pub fn part_two(input: &str) -> Option<usize> {
    let ops: Operations = input.lines().try_into().unwrap();

    let mut zeros = 0;
    let mut state = LockState::default();

    for op in ops.as_ref() {
        let delta = state.apply(op);
        zeros += delta;
    }

    Some(zeros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_overflow() {
        let mut state = LockState(95);

        let zeros = state.apply(&Operation::Right(10));
        let after: usize = (&state).into();

        assert_eq!(after, 5);
        assert_eq!(zeros, 1);
    }

    #[test]
    fn apply_underflow() {
        let mut state = LockState(5);

        let zeros = state.apply(&Operation::Left(10));
        let after: usize = (&state).into();

        assert_eq!(after, 95);
        assert_eq!(zeros, 1);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
