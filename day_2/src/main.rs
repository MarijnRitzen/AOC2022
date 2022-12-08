use std::{fs, cmp::Ordering};

#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}
impl Shape {
    fn get_score(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn opponent_should_be(&self, outcome: std::cmp::Ordering) -> &Shape {
        match outcome {
            Ordering::Less => {
                match self {
                    Shape::Rock => &Shape::Scissors,
                    Shape::Paper => &Shape::Rock,
                    Shape::Scissors => &Shape::Paper,
                }
            },
            Ordering::Equal => {
                match self {
                    Shape::Rock => &Shape::Rock,
                    Shape::Paper => &Shape::Paper,
                    Shape::Scissors => &Shape::Scissors,
                }
            }
            Ordering::Greater => {
                match self {
                    Shape::Rock => &Shape::Paper,
                    Shape::Paper => &Shape::Scissors,
                    Shape::Scissors => &Shape::Rock,
                }
            }
        }
    }
}

impl From<&str> for Shape {
    fn from(char: &str) -> Self {
        match char {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Unrecognized shape: {}", char)
        } 
    }


}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Shape::Rock => {
                match other {
                    Shape::Rock => Some(std::cmp::Ordering::Equal),
                    Shape::Paper => Some(std::cmp::Ordering::Less),
                    Shape::Scissors => Some(std::cmp::Ordering::Greater),
                }
            },
            Shape::Paper => {
                match other {
                    Shape::Rock => Some(std::cmp::Ordering::Greater),
                    Shape::Paper => Some(std::cmp::Ordering::Equal),
                    Shape::Scissors => Some(std::cmp::Ordering::Less),
                }
            },
            Shape::Scissors => {
                match other {
                    Shape::Rock => Some(std::cmp::Ordering::Less),
                    Shape::Paper => Some(std::cmp::Ordering::Greater),
                    Shape::Scissors => Some(std::cmp::Ordering::Equal),
                }
            }
        }
    }
}

fn main() {
    let string: String = fs::read_to_string("input.txt").unwrap();

    let mut total_score: usize = 0;
    for line in string.lines() {
        if let [opponent, outcome] = line.split(' ').collect::<Vec<_>>().as_slice() {
            let outcome_ord = match outcome {
                &"X" => Ordering::Less,
                &"Y" => {
                    total_score += 3;
                    Ordering::Equal
                }
                &"Z" => {
                    total_score += 6;
                    Ordering::Greater
                }
                _ => unreachable!()
            };
            
            let opponent: Shape = (*opponent).into();

            total_score += opponent.opponent_should_be(outcome_ord).get_score();
        }
    }

    println!("Total score: {}", total_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        assert!(Shape::Paper > Shape::Rock);
        assert!(Shape::Paper == Shape::Paper);
        assert!(Shape::Rock > Shape::Scissors);
        assert!(Shape::Scissors > Shape::Paper);
        
        assert_eq!(Shape::Scissors.partial_cmp(&Shape::Paper), Some(std::cmp::Ordering::Greater));
        assert_eq!(Shape::Scissors.partial_cmp(&Shape::Scissors), Some(std::cmp::Ordering::Equal));
    }
}
