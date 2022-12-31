use std::fs;

fn get_end_unique_window(input: &str, window_size: usize) -> usize {
    for (index, window) in input.as_bytes().windows(window_size).enumerate() {
        let mut window = window.to_vec();
        window.sort(); 
        window.dedup();
        
        if window.len() == window_size {
            return index + window_size;
        }
    }

    input.as_bytes().len()
}

fn get_start_of_packet_marker(input: &str) -> usize {
    get_end_unique_window(input, 4)
}

fn get_start_of_message_marker(input: &str) -> usize {
    get_end_unique_window(input, 14)
}

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let result = get_start_of_message_marker(&input);

    println!("{result}");
}

#[cfg(test)]
mod tests {
    use crate::*;


    #[test]
    fn test_part_one() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        assert_eq!(get_start_of_packet_marker(input), 5);
    }

    #[test]
    fn test_part_two() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        assert_eq!(get_start_of_message_marker(input), 19);
    }
}
