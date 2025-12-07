advent_of_code::solution!(7);

fn do_the_beams_thing(input: &str) -> (usize, Vec<Option<usize>>) {
    let mut splits = 0;
    let length = input.lines().next().unwrap().len();
    let mut beams: Vec<Option<usize>> = (0..length).map(|_| None).collect();
    for line in input.lines() {
        for (idx, char) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
            match char {
                'S' => {
                    beams[idx] = Some(1);
                }
                '^' => {
                    if let Some(value) = beams[idx] {
                        // split
                        splits += 1;
                        beams[idx] = None;

                        let left = beams[idx - 1].unwrap_or(0) + value;
                        beams[idx - 1] = Some(left);

                        let right = beams[idx + 1].unwrap_or(0) + value;
                        beams[idx + 1] = Some(right);
                    }
                }
                invalid => panic!("unexpected input: {invalid}"),
            }
        }
    }

    (splits, beams)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (splits, _) = do_the_beams_thing(input);
    splits.into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, beams) = do_the_beams_thing(input);
    beams.into_iter().flatten().sum::<usize>().into()
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
