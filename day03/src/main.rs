use std::fs;


fn get_char_priority(character: u8) -> u8 {
    if character >= b'a' {
        character - b'a' + 1
    } else {
        character - b'A' + 27
    }
}
fn main() {
    let string: String = fs::read_to_string("input.txt").unwrap();

    let mut result: u64 = 0;
    let strings: Vec<_> = string.lines().map(|l| l.trim()).collect();
    let strings_slice = &strings;
    let iter = strings_slice.chunks(3);

    for triplet in iter {
        let first = triplet[0];
        let second = triplet[1];
        let third = triplet[2];

        'outer: for first_byte in first.as_bytes() {
            for second_byte in second.as_bytes() {
                for third_byte in third.as_bytes() {
                    if first_byte == second_byte && second_byte == third_byte {
                        result += get_char_priority(*first_byte) as u64;
                        break 'outer;
                    }
                }
            }
        }
    }

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use crate::get_char_priority;

    #[test]
    fn test_split_str() {
        let string = "abab";

        let tuple = string.split_at(string.len() / 2);
        assert_eq!(("ab", "ab"), tuple);
    }

    #[test]
    fn test_byte_to_priority() {
        assert_eq!(get_char_priority(b'a'), 1);
        assert_eq!(get_char_priority(b'c'), 3);
        assert_eq!(get_char_priority(b'z'), 26);
        assert_eq!(get_char_priority(b'A'), 27);
        assert_eq!(get_char_priority(b'Z'), 52);
    }
}
