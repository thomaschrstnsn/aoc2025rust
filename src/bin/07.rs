use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let mut num_splits = 0;

    let mut beams = HashSet::new();
    for line in input.lines() {
        for (idx, char) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
            match char {
                'S' => {
                    beams.insert(idx);
                }
                '^' => {
                    if beams.contains(&idx) {
                        // split
                        num_splits += 1;
                        beams.remove(&idx);
                        beams.insert(idx - 1);
                        beams.insert(idx + 1);
                    }
                }
                invalid => panic!("unexpected input: {invalid}"),
            }
        }
    }

    Some(num_splits)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut beams: HashMap<usize, usize> = HashMap::new();
    for line in input.lines() {
        for (idx, char) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
            match char {
                'S' => {
                    beams.insert(idx, 1);
                }
                '^' => {
                    if let Some(value) = beams.get(&idx).cloned() {
                        // split
                        beams.remove(&idx);

                        beams
                            .entry(idx - 1)
                            .and_modify(|v| *v += value)
                            .or_insert(value);

                        beams
                            .entry(idx + 1)
                            .and_modify(|v| *v += value)
                            .or_insert(value);
                    }
                }
                invalid => panic!("unexpected input: {invalid}"),
            }
        }
    }

    beams.values().sum::<usize>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
