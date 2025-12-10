use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

advent_of_code::solution!(10);

#[derive(Debug)]
struct IndicatorLights(Vec<bool>);

#[derive(Debug, Clone, Copy)]
struct BitField(u16);

impl BitField {
    /// Create a new empty bitfield (all bits = 0)
    fn new() -> Self {
        Self(0)
    }

    /// Set a bit at index (0–15)
    fn set(&mut self, index: u8) {
        self.0 |= 1 << index;
    }

    /// Extract the bits as a u16
    fn as_u16(&self) -> u16 {
        self.0
    }
}

impl IndicatorLights {
    fn as_u16(&self) -> u16 {
        let mut res = BitField::new();
        for (idx, b) in self.0.iter().enumerate() {
            if *b {
                res.set(idx as u8);
            }
        }

        res.as_u16()
    }
}

#[derive(Debug, Error)]
enum IndicatorLightParsingError {
    #[error("Invalid format for indicator light, found: {found} expected one of: {expected:?}")]
    InvalidFormat { found: String, expected: Vec<char> },
}

impl FromStr for IndicatorLights {
    type Err = IndicatorLightParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s
            .split_once('[')
            .ok_or(IndicatorLightParsingError::InvalidFormat {
                found: s.into(),
                expected: vec!['['],
            })?;

        let (s, _) = s
            .split_once(']')
            .ok_or(IndicatorLightParsingError::InvalidFormat {
                found: s.into(),
                expected: vec![']'],
            })?;

        let lights = s
            .chars()
            .map(|c| match c {
                '.' => Ok(false),
                '#' => Ok(true),
                invalid => Err(IndicatorLightParsingError::InvalidFormat {
                    found: invalid.into(),
                    expected: vec!['.', '#'],
                }),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(lights))
    }
}

#[derive(Debug)]
struct SingleWiringSchematic(Vec<u8>);

impl SingleWiringSchematic {
    fn as_u16(&self) -> u16 {
        let mut bf = BitField::new();
        for &num in &self.0 {
            bf.set(num);
        }
        bf.0
    }
}

#[derive(Debug, Error)]
enum WiringSchematicParseError {
    #[error("Invalid format, found: {found} expected: {expected:?}")]
    InvalidFormat { found: String, expected: Vec<char> },

    #[error("Invalid number")]
    InvalidNumber(#[from] ParseIntError),
}

impl FromStr for SingleWiringSchematic {
    type Err = WiringSchematicParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s
            .split_once('(')
            .ok_or(WiringSchematicParseError::InvalidFormat {
                found: s.into(),
                expected: vec!['('],
            })?;

        let (s, _) = s
            .split_once(')')
            .ok_or(WiringSchematicParseError::InvalidFormat {
                found: s.into(),
                expected: vec![')'],
            })?;

        let nums = s
            .split(',')
            .map(|n| n.parse())
            .collect::<Result<Vec<u8>, _>>()?;

        Ok(Self(nums))
    }
}

#[derive(Debug)]
struct JoltageReqs(Vec<usize>);

#[derive(Debug, Error)]
enum JoltageReqsParseError {
    #[error("Invalid format, found: {found} expected: {expected:?}")]
    InvalidFormat { found: String, expected: Vec<char> },

    #[error("Invalid number")]
    InvalidNumber(#[from] ParseIntError),
}

impl FromStr for JoltageReqs {
    type Err = JoltageReqsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s
            .split_once('{')
            .ok_or(JoltageReqsParseError::InvalidFormat {
                found: s.into(),
                expected: vec!['{'],
            })?;

        let (s, _) = s
            .split_once('}')
            .ok_or(JoltageReqsParseError::InvalidFormat {
                found: s.into(),
                expected: vec!['}'],
            })?;

        let nums = s
            .split(',')
            .map(|n| n.parse())
            .collect::<Result<Vec<usize>, _>>()?;

        Ok(Self(nums))
    }
}

#[derive(Debug)]
struct Machine {
    indicator_lights: IndicatorLights,
    button_wiring: Vec<SingleWiringSchematic>,
    joltage: JoltageReqs,
}

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::ops::Range;

fn min_sum_with_predicate<F>(ranges: &[Range<usize>], mut pred: F) -> Option<(usize, Vec<usize>)>
where
    F: FnMut(&[usize]) -> bool,
{
    if ranges.is_empty() {
        return None;
    }

    // Initial combination: all starts
    let start: Vec<usize> = ranges.iter().map(|r| r.start).collect();
    let start_sum: usize = start.iter().copied().sum();

    // Min-heap via Reverse (BinaryHeap is a max-heap by default)
    let mut heap: BinaryHeap<(Reverse<usize>, Vec<usize>)> = BinaryHeap::new();
    let mut seen: HashSet<Vec<usize>> = HashSet::new();

    heap.push((Reverse(start_sum), start.clone()));
    seen.insert(start.clone());

    while let Some((Reverse(sum), combo)) = heap.pop() {
        // Check predicate on the current best-sum combination
        if pred(&combo) {
            return Some((sum, combo));
        }

        // Generate neighbors by bumping one coordinate
        for dim in 0..ranges.len() {
            let mut next = combo.clone();
            next[dim] += 1;

            if next[dim] < ranges[dim].end && seen.insert(next.clone()) {
                // Update sum incrementally:
                let next_sum = sum - combo[dim] + next[dim];
                heap.push((Reverse(next_sum), next));
            }
        }
    }

    None
}

impl Machine {
    fn target(&self) -> u16 {
        self.indicator_lights.as_u16()
    }

    fn fewest_button_presses(&self) -> (usize, Vec<usize>) {
        let ranges = (0..self.button_wiring.len())
            .map(|_| 0..3)
            .collect::<Vec<_>>();

        let masks = self
            .button_wiring
            .iter()
            .map(|bw| bw.as_u16())
            .collect::<Vec<_>>();

        let result = min_sum_with_predicate(&ranges, |vals| self.test_button_setup(vals, &masks));

        result.unwrap()
    }

    fn test_button_setup(&self, setup: &[usize], masks: &[u16]) -> bool {
        let mut state = 0u16;
        let target = self.target();

        for (button_idx, &repeats) in setup.iter().enumerate() {
            let mask = masks[button_idx];

            for _ in 0..repeats {
                state ^= mask;
            }
        }

        state == target
    }
}

#[derive(Debug, Error)]
enum MachineParsingError {
    #[error("Invalid indicator lights")]
    IndicatorLights(#[from] IndicatorLightParsingError),
    #[error("Invalid wiring schematics lights")]
    WiringSchematics(#[from] WiringSchematicParseError),
    #[error("Invalid joltage requirements")]
    JoltageReqs(#[from] JoltageReqsParseError),
    #[error("missing indicator lights")]
    MissingIndicatorLights,
    #[error("missing joltage")]
    MissingJoltage,
    #[error("left over after reading value: {leftover}")]
    UnexpectedLeftover { leftover: String },
}

impl FromStr for Machine {
    type Err = MachineParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elems = s.split_whitespace();

        let indicator_lights: IndicatorLights = elems
            .next()
            .ok_or(MachineParsingError::MissingIndicatorLights)?
            .parse()?;

        let mut wiring_schematics: Vec<SingleWiringSchematic> = Vec::new();
        let mut joltage: Option<JoltageReqs> = None;
        for el in elems.by_ref() {
            if let Ok(schematic) = el.parse::<SingleWiringSchematic>() {
                wiring_schematics.push(schematic);
            } else {
                let jolta: JoltageReqs = el.parse()?;

                joltage = Some(jolta);

                break;
            }
        }

        if let Some(leftover) = elems.next() {
            return Err(MachineParsingError::UnexpectedLeftover {
                leftover: leftover.into(),
            });
        }

        let joltage = if let Some(joltage) = joltage {
            joltage
        } else {
            return Err(MachineParsingError::MissingJoltage);
        };

        Ok(Self {
            indicator_lights,
            button_wiring: wiring_schematics,
            joltage,
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let machines = input
        .lines()
        .map(|line| line.parse::<Machine>())
        .collect::<Result<Vec<_>, _>>()
        .expect("can parse");

    let fewest = machines.iter().map(|m| m.fewest_button_presses().0).sum();

    Some(fewest)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let file = advent_of_code::template::read_file("examples", DAY);
        let line = file.lines().next().unwrap();
        let machine: Machine = line.parse().expect("parses");

        let actual = machine.fewest_button_presses();
        dbg!(&actual.1);
        assert_eq!(actual.0, 2);
    }

    #[test]
    fn lights_6_to_u16() {
        let lights = IndicatorLights(vec![false, true, true, false]);

        let actual = lights.as_u16();
        assert_eq!(actual, 6);
    }

    #[test]
    fn lights_1_to_u16() {
        let lights = IndicatorLights(vec![true, false]);

        let actual = lights.as_u16();
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
