use std::{ops::Deref, str::FromStr};

use thiserror::Error;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Row(Vec<bool>);

impl Deref for Row {
    type Target = [bool];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Grid(Vec<Row>);

trait AddingOffset: Sized {
    fn add_offset(&self, offset: isize) -> Option<Self>;
}

impl AddingOffset for usize {
    fn add_offset(&self, off: isize) -> Option<Self> {
        if off >= 0 {
            self.checked_add(off as usize)
        } else {
            self.checked_sub((-off) as usize)
        }
    }
}

impl Grid {
    const fn neighbors_8_offset(&self) -> [(isize, isize); 8] {
        [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ]
    }

    fn get_cell(&self, row: usize, col: usize) -> Option<bool> {
        if row < self.0.len() && col < self.0[0].0.len() {
            Some(self.0[row].0[col])
        } else {
            None
        }
    }

    fn set_cell(&mut self, row: usize, col: usize, value: bool) {
        if row < self.0.len() && col < self.0[0].0.len() {
            self.0[row].0[col] = value;
        }
    }

    fn count_neighbors_8(&self, row: usize, col: usize) -> usize {
        let offsets = self.neighbors_8_offset();
        let mut count = 0;

        for (o_row, o_col) in offsets.iter() {
            if let Some(n_row) = row.add_offset(*o_row)
                && let Some(n_col) = col.add_offset(*o_col)
                && let Some(true) = self.get_cell(n_row, n_col)
            {
                count += 1;
            }
        }

        count
    }

    fn indices(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.0.len()).flat_map(move |row| (0..self.0[0].0.len()).map(move |col| (row, col)))
    }
}

impl AsRef<Vec<Row>> for Grid {
    fn as_ref(&self) -> &Vec<Row> {
        &self.0
    }
}

#[derive(Debug, Error)]
enum GridParseError {
    #[error("Unexpected input: {unexpected} was looking for @ or .")]
    UnexpectedInput { unexpected: char },
}

impl FromStr for Row {
    type Err = GridParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .chars()
            .map(|c| match c {
                '@' => Ok(true),
                '.' => Ok(false),
                invalid => Err(GridParseError::UnexpectedInput {
                    unexpected: invalid,
                }),
            })
            .collect::<Result<Vec<_>, _>>();

        Ok(Self(cells?))
    }
}

impl FromStr for Grid {
    type Err = GridParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines().map(|r| r.parse()).collect::<Result<Vec<_>, _>>();
        Ok(Self(rows?))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid = input.parse().expect("parses");

    let mut sum = 0;
    for (row, col) in grid.indices() {
        if let Some(true) = grid.get_cell(row, col) {
            let count = grid.count_neighbors_8(row, col);
            if count < 4 {
                sum += 1
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid: Grid = input.parse().expect("parses");

    let mut sum = 0;
    loop {
        let this_round = remove_one_round(&mut grid);
        sum += this_round;
        if this_round == 0 {
            break;
        }
    }

    Some(sum)
}

fn remove_one_round(grid: &mut Grid) -> usize {
    let mut count = 0;
    let mut indices_to_remove = vec![];
    for (row, col) in grid.indices() {
        if let Some(true) = grid.get_cell(row, col) {
            let neighbors = grid.count_neighbors_8(row, col);
            if neighbors < 4 {
                count += 1;
                indices_to_remove.push((row, col));
            }
        }
    }

    for (row, col) in indices_to_remove {
        debug_assert!(grid.get_cell(row, col) == Some(true));
        grid.set_cell(row, col, false);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
