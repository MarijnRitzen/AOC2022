use std::{fs, str::FromStr};

#[derive(Debug, PartialEq)]
struct Range {
    low: u32,
    high: u32
}

impl Range {

    fn contains(&self, other: &Range) -> bool {
        (self.low <= other.low && other.high <= self.high) || (other.low <= self.low && self.high <= other.high)
    }

    fn overlap(&self, other: &Range) -> bool {
        (self.low <= other.low && other.low <= self.high) || (other.low <= self.low && self.low <= other.high)
    }
}

impl FromStr for Range {
    type Err = std::num::ParseIntError;

    // Expects a-b
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (low_str, high_str) = s.split_once('-').unwrap();

        let low = low_str.parse().unwrap();
        let high = high_str.parse().unwrap();

        Ok(Range {low, high})
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let mut contained = 0;
    let mut overlap = 0;

    for line in input.lines() {
        let (first_range, second_range) = line.split_once(',').unwrap();
        let (first_range, second_range) : (Range, Range) = (first_range.parse().unwrap(), second_range.parse().unwrap());

        if first_range.contains(&second_range) || second_range.contains(&first_range) {
            contained += 1;
        }

        if first_range.overlap(&second_range) || second_range.overlap(&first_range) {
            overlap += 1;
        }
    }

    println!("Contained: {contained}, Overlap: {overlap}");
}
