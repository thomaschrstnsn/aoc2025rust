use std::{collections::BinaryHeap, fmt::Display, str::FromStr};

advent_of_code::solution!(11);

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
struct DeviceId([u8; 3]);

impl DeviceId {
    pub fn start() -> Self {
        const YOU: &[u8; 3] = b"you";
        Self(*YOU)
    }

    pub fn end() -> Self {
        const OUT: &[u8; 3] = b"out";
        Self(*OUT)
    }
}

impl Display for DeviceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.0).map_err(|_| std::fmt::Error)?;
        write!(f, "{s}")
    }
}

impl std::fmt::Debug for DeviceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.0).map_err(|_| std::fmt::Error)?;
        write!(f, "DeviceId({s})")
    }
}

#[derive(Debug, Eq, PartialEq)]
struct DeviceAttachment(DeviceId, Vec<DeviceId>);

impl DeviceAttachment {
    pub fn device(&self) -> &DeviceId {
        &self.0
    }

    pub fn attachments(&self) -> &Vec<DeviceId> {
        &self.1
    }
}

#[derive(Debug, thiserror::Error)]
enum DeviceParsingError {
    #[error("Invalid format for device ID")]
    InvalidFormat,
}

impl FromStr for DeviceId {
    type Err = DeviceParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 || s.chars().any(|c| !c.is_alphabetic()) {
            return Err(DeviceParsingError::InvalidFormat);
        }

        if let &[a, b, c] = s.as_bytes() {
            Ok(DeviceId([a, b, c]))
        } else {
            Err(DeviceParsingError::InvalidFormat)
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum DeviceAttachmentParseError {
    #[error("Invalid format for device attachment")]
    InvalidFormat,

    #[error("DeviceId broken")]
    DeviceId(#[from] DeviceParsingError),
}

impl FromStr for DeviceAttachment {
    type Err = DeviceAttachmentParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dev, attachments) = s
            .split_once(":")
            .ok_or(DeviceAttachmentParseError::InvalidFormat)?;

        let dev: DeviceId = dev.parse()?;

        let attachments = attachments
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<DeviceId>, DeviceParsingError>>()?;

        Ok(Self(dev, attachments))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let graph = input
        .lines()
        .map(|line| line.parse::<DeviceAttachment>())
        .collect::<Result<Vec<DeviceAttachment>, DeviceAttachmentParseError>>()
        .expect("can parse");

    let mut working_set = BinaryHeap::with_capacity(graph.len());
    working_set.push(DeviceId::start());

    let mut result = 0;
    while let Some(item) = working_set.pop() {
        let entry = graph
            .iter()
            .find(|g| *g.device() == item)
            .expect("entry is in graph");

        for addition in entry.attachments().iter() {
            if *addition == DeviceId::end() {
                result += 1;
            } else {
                // seen.push(addition);
                working_set.push(*addition);
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
