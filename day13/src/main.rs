use std::fs;

#[derive(PartialEq, Debug, Clone, Eq)]
enum Packet {
    List(Vec<Packet>),
    Number(usize),
}

impl Packet {
    fn parse(string: &str) -> Packet {
        if let Ok(nr) = string.parse() {
            return Packet::Number(nr);
        }

        // From here on we can assume we are working with a list
        // We will split the list in its elements and parse those
        // and return Packet::List containing those

        let mut parts: Vec<&str> = Vec::new();
        let mut balance = 0;
        let mut end_last_part = 1;

        for (index, byte) in string.bytes().enumerate() {
            match byte {
                b'[' => balance += 1,
                b']' => {
                    if balance == 1 && end_last_part != index {
                        parts.push(&string[end_last_part..index].trim_start_matches(","));
                        end_last_part = index;
                    }

                    balance -= 1;
                }
                b',' => {
                    if balance == 1 {
                        parts.push(&string[end_last_part..index].trim_start_matches(","));
                        end_last_part = index;
                    }
                }
                _ => (),
            };
        }

        Packet::List(parts.iter().map(|s| Self::parse(s)).collect())
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(left), Packet::List(right)) => {
                for index in 0..std::cmp::min(left.len(), right.len()) {
                    match left[index].cmp(&right[index]) {
                        std::cmp::Ordering::Equal => continue,
                        other => return other,
                    }
                }

                left.len().cmp(&right.len())
            }
            (Packet::List(left), Packet::Number(right)) => {
                Packet::List(left.to_vec()).cmp(&Packet::List(vec![Packet::Number(*right)]))
            }
            (Packet::Number(left), Packet::List(right)) => {
                Packet::List(vec![Packet::Number(*left)]).cmp(&Packet::List(right.to_vec()))
            }
            (Packet::Number(left), Packet::Number(right)) => left.cmp(right),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct DistressSignal {}

impl DistressSignal {
    #[allow(dead_code)]
    fn part_one(input: String) -> usize {
        let mut result = 0;

        for (index, pair) in input.split("\n\n").enumerate() {
            let left = pair.split_once("\n").unwrap().0;
            let right = pair.split_once("\n").unwrap().1;

            if Packet::parse(left) < Packet::parse(right) {
                result += index + 1;
            }
        }

        result
    }

    #[allow(dead_code)]
    fn part_two(input: String) -> usize {
        let mut packet_list: Vec<Packet> = input
            .split_whitespace()
            .map(|string| Packet::parse(string))
            .collect();

        packet_list.push(Packet::parse("[[2]]"));
        packet_list.push(Packet::parse("[[6]]"));

        packet_list.sort();

        (packet_list
            .iter()
            .position(|packet| *packet == Packet::List(vec![Packet::List(vec![Packet::Number(2)])]))
            .unwrap()
            + 1)
            * (packet_list
                .iter()
                .position(|packet| {
                    *packet == Packet::List(vec![Packet::List(vec![Packet::Number(6)])])
                })
                .unwrap()
                + 1)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // println!("Result part one: {}", DistressSignal::part_one(input));
    println!("Result part two: {}", DistressSignal::part_two(input));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{DistressSignal, Packet};

    #[test]
    fn test_basic_number() {
        assert_eq!(Packet::parse("1"), Packet::Number(1));
    }

    #[test]
    fn test_empty_list() {
        assert_eq!(Packet::parse("[]"), Packet::List(vec![]));
    }

    #[test]
    fn test_list_with_one_item() {
        assert_eq!(Packet::parse("[1]"), Packet::List(vec![Packet::Number(1)]));
    }

    #[test]
    fn test_nested_list() {
        assert_eq!(
            Packet::parse("[[]]"),
            Packet::List(vec![Packet::List(vec![])])
        );
    }

    #[test]
    fn test_trim_start() {
        let string = ",a";
        assert_eq!("a", string.trim_start_matches(","));
    }

    #[test]
    fn test_complicated() {
        assert_eq!(
            Packet::parse("[1,2,[3,[4]]]"),
            Packet::List(vec![
                Packet::Number(1),
                Packet::Number(2),
                Packet::List(vec![
                    Packet::Number(3),
                    Packet::List(vec![Packet::Number(4)])
                ]),
            ])
        );
    }

    #[test]
    fn test_sorting_1() {
        assert_eq!(
            Packet::parse("[1,1,3,1,1]").cmp(&Packet::parse("[1,1,5,1,1]")),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn test_sorting_2() {
        assert_eq!(
            Packet::parse("[[1],[2,3,4]]").cmp(&Packet::parse("[[1],4]")),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn test_sorting_3() {
        assert_eq!(
            Packet::parse("[9]").cmp(&Packet::parse("[[8,7,6]]")),
            std::cmp::Ordering::Greater
        );
    }

    #[test]
    fn test_sorting_4() {
        assert_eq!(
            Packet::parse("[[4,4],4,4]").cmp(&Packet::parse("[[4,4],4,4,4]")),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn test_sorting_5() {
        assert_eq!(
            Packet::parse("[7,7,7,7]").cmp(&Packet::parse("[7,7,7]")),
            std::cmp::Ordering::Greater
        );
    }

    #[test]
    fn test_sorting_6() {
        assert_eq!(
            Packet::parse("[]").cmp(&Packet::parse("[3]")),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn test_sorting_7() {
        assert_eq!(
            Packet::parse("[[[]]]").cmp(&Packet::parse("[[]]")),
            std::cmp::Ordering::Greater
        );
    }

    #[test]
    fn test_sorting_8() {
        assert_eq!(
            Packet::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]")
                .cmp(&Packet::parse("[1,[2,[3,[4,[5,6,0]]]],8,9]")),
            std::cmp::Ordering::Greater
        );
    }

    #[test]
    fn test_input_part_one() {
        let input = fs::read_to_string("test_input.txt").unwrap();

        assert_eq!(DistressSignal::part_one(input), 13);
    }

    #[test]
    fn test_input_part_two() {
        let input = fs::read_to_string("test_input.txt").unwrap();

        assert_eq!(DistressSignal::part_two(input), 140);
    }
}
