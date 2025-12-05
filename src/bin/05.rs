use std::{
    ops::{Deref, RangeInclusive},
    str::FromStr,
};

use thiserror::Error;

advent_of_code::solution!(5);

#[derive(Debug, Clone)]
struct FreshIngredients(RangeInclusive<usize>);

enum Combination {
    Merged,
    NoOverlap,
}

impl FreshIngredients {
    fn combine(&mut self, other: &Self) -> Combination {
        if self.0.contains(other.start()) && self.0.contains(other.end()) {
            Combination::Merged
        } else if other.0.contains(self.start()) && other.0.contains(self.end()) {
            self.0 = other.0.clone();
            Combination::Merged
        } else if self.0.contains(other.start()) {
            self.0 = *self.start()..=*other.end();
            Combination::Merged
        } else if self.0.contains(other.end()) {
            self.0 = *other.start()..=*self.end();
            Combination::Merged
        } else {
            Combination::NoOverlap
        }
    }

    fn len(&self) -> usize {
        self.0.end() - self.0.start() + 1
    }
}

impl Deref for FreshIngredients {
    type Target = RangeInclusive<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for FreshIngredients {
    type Err = DatabaseParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s
            .split_once('-')
            .ok_or(DatabaseParsingError::InvalidRange {
                found: s.to_string(),
            })?;

        let from = from.parse::<usize>()?;
        let to = to.parse::<usize>()?;

        Ok(FreshIngredients(from..=to))
    }
}

#[derive(Debug)]
struct Database {
    fresh_ingredients: Vec<FreshIngredients>,
    available_ingredients: Vec<usize>,
}

impl Database {
    fn find_fresh_and_available_ingredients(&self) -> impl Iterator<Item = usize> {
        self.available_ingredients
            .iter()
            .filter(|&i| self.is_fresh(*i))
            .cloned()
    }

    fn compact_fresh_ingredients(&mut self) -> bool {
        let mut combined: Vec<FreshIngredients> = vec![self.fresh_ingredients[0].clone()];

        let mut did_absorb = false;
        for range in self.fresh_ingredients.iter().skip(1) {
            let mut absorbed = false;
            for candidate in combined.iter_mut() {
                match candidate.combine(range) {
                    Combination::Merged => {
                        absorbed = true;
                        did_absorb = true;
                        break;
                    }
                    Combination::NoOverlap => continue,
                }
            }

            if !absorbed {
                combined.push(range.clone());
            }
        }

        if did_absorb {
            self.fresh_ingredients = combined;
        }

        did_absorb
    }

    fn is_fresh(&self, ingredient: usize) -> bool {
        self.fresh_ingredients
            .iter()
            .any(|range| range.contains(&ingredient))
    }
}

#[derive(Debug, Error)]
enum DatabaseParsingError {
    #[error("invalid range, expected format 'start-end', found {found}")]
    InvalidRange { found: String },

    #[error("Ingredient parsing error: {0}")]
    InvalidIngredient(#[from] std::num::ParseIntError),
}

impl FromStr for Database {
    type Err = DatabaseParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let mut fresh_ingredients = Vec::new();
        for line in lines.by_ref() {
            match line.parse::<FreshIngredients>() {
                Ok(fresh) => fresh_ingredients.push(fresh),
                Err(err) => {
                    if line.trim().is_empty() {
                        break;
                    } else {
                        return Err(err);
                    }
                }
            }
        }

        let available_ingredients = lines
            .map(|l| l.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            fresh_ingredients,
            available_ingredients,
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut db: Database = input.parse().expect("valid database");

    while db.compact_fresh_ingredients() {}

    let fresh_ingredients = db.find_fresh_and_available_ingredients();

    Some(fresh_ingredients.count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut db: Database = input.parse().expect("valid database");

    while db.compact_fresh_ingredients() {}

    let count = db.fresh_ingredients.iter().map(|r| r.len()).sum();

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_len() {
        assert_eq!(FreshIngredients(2..=5).len(), 4);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
