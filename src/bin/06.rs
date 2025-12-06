use std::{marker::PhantomData, str::FromStr};

use thiserror::Error;

advent_of_code::solution!(6);

#[derive(Clone, Debug, Copy)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
struct PartOne;

#[derive(Debug)]
struct PartTwo;

#[derive(Debug)]
struct MathProblem<T> {
    input: Vec<usize>,
    op: Operator,
    marker: PhantomData<T>,
}

impl<T> MathProblem<T> {
    fn result(&self) -> usize {
        match self.op {
            Operator::Add => self.input.iter().sum(),
            Operator::Multiply => self.input.iter().product(),
        }
    }
}

#[derive(Debug)]
struct MathProblemSet<T>(Vec<MathProblem<T>>);

#[derive(Debug, Error)]
enum MathProblemParseError {
    #[error("invalid operator, found: {found} expected + or *")]
    InvalidOperator { found: String },

    #[error("invalid number")]
    InvalidNumber(#[from] std::num::ParseIntError),

    #[error("mismatch in problem sizes, saw: {problem} expected: {expected}")]
    MismatchInProblemSizes { problem: usize, expected: usize },

    #[error("mismatch in column widths, saw: {problem} expected: {expected}")]
    ColumnMismatch { problem: usize, expected: usize },

    #[error("number after operator")]
    NumberAfterOperator,
}

impl FromStr for MathProblemSet<PartOne> {
    type Err = MathProblemParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let cols: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|el| el.parse())
            .collect::<Result<Vec<_>, _>>()?;

        let width = cols.len();

        let mut results: Vec<Vec<usize>> = Vec::with_capacity(width);

        for c in cols.into_iter() {
            results.push(vec![c])
        }

        let mut operators: Vec<Operator> = Vec::with_capacity(width);

        let mut operator_seen = false;
        for line in lines {
            let elems = line.split_whitespace();
            for (index, el) in elems.enumerate() {
                let num: Result<usize, std::num::ParseIntError> = el.parse();
                let op: Result<Operator, MathProblemParseError> = match el {
                    "+" => Ok(Operator::Add),
                    "*" => Ok(Operator::Multiply),
                    _ => Err(MathProblemParseError::InvalidOperator {
                        found: el.to_string(),
                    }),
                };

                if num.is_ok() && operator_seen {
                    return Err(MathProblemParseError::NumberAfterOperator);
                }
                if op.is_ok() && !operator_seen {
                    operator_seen = true;
                }

                if let Ok(op) = op {
                    operators.push(op)
                } else if let Ok(num) = num {
                    results[index].push(num);
                } else if operator_seen {
                    return Err(op.err().unwrap());
                } else {
                    return Err(MathProblemParseError::InvalidNumber(num.err().unwrap()));
                }
            }
        }

        if operators.len() != width {
            return Err(MathProblemParseError::MismatchInProblemSizes {
                problem: operators.len(),
                expected: width,
            });
        }

        let expected_len = results.first().unwrap().len();

        let mut problems = Vec::with_capacity(width);
        for (index, problem) in results.into_iter().enumerate() {
            if problem.len() != expected_len {
                return Err(MathProblemParseError::ColumnMismatch {
                    problem: problem.len(),
                    expected: expected_len,
                });
            }

            let problem = MathProblem::<PartOne> {
                input: problem,
                op: operators[index].clone(),
                marker: PhantomData,
            };

            problems.push(problem);
        }

        Ok(Self(problems))
    }
}

impl FromStr for MathProblemSet<PartTwo> {
    type Err = MathProblemParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_rev = s.lines().rev();

        let op_line = lines_rev.next().unwrap();

        let indices_and_operators: Vec<(usize, Operator)> = op_line
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| {
                let maybe_op = match c {
                    '+' => Some(Operator::Add),
                    '*' => Some(Operator::Multiply),
                    ' ' => None,
                    _ => panic!("this should be handled better 😱"),
                };

                maybe_op.map(|op| (idx, op))
            })
            .collect();

        let number_of_problems = indices_and_operators.len();

        let mut problem_numbers: Vec<Vec<String>> =
            indices_and_operators.iter().map(|_| Vec::new()).collect();
        for line in s.lines() {
            for (problem_idx, (string_idx, _)) in indices_and_operators.iter().enumerate() {
                let (_, nums) = line.split_at(*string_idx);

                let mut seen_digit = false;
                for (num_idx, num_char) in nums.chars().enumerate() {
                    let problem_nums = problem_numbers.get_mut(problem_idx).unwrap();

                    if num_char.is_ascii_digit() {
                        seen_digit = true;
                        while problem_nums.get(num_idx).is_none() {
                            problem_nums.push(String::new());
                        }
                        problem_nums
                            .get_mut(num_idx)
                            .unwrap()
                            .push_str(&num_char.to_string());
                    } else if num_char.is_ascii_whitespace() && seen_digit {
                        break;
                    }
                }
            }
        }

        if number_of_problems != problem_numbers.len() {
            return Err(MathProblemParseError::MismatchInProblemSizes {
                problem: problem_numbers.len(),
                expected: number_of_problems,
            });
        }

        let mut problems = Vec::with_capacity(number_of_problems);
        for (index, prob) in problem_numbers.iter().enumerate() {
            let numbers = prob
                .iter()
                .map(|s| s.parse())
                .collect::<Result<Vec<usize>, _>>()?;

            problems.push(MathProblem::<PartTwo> {
                input: numbers,
                op: indices_and_operators.get(index).unwrap().1,
                marker: PhantomData,
            });
        }

        Ok(Self(problems))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let problems: MathProblemSet<PartOne> = input.parse().expect("can parse");

    problems.0.iter().map(|p| p.result()).sum::<usize>().into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let problems: MathProblemSet<PartTwo> = input.parse().expect("can parse");

    problems.0.iter().map(|p| p.result()).sum::<usize>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
