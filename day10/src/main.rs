use std::{fs, fmt::Debug};

struct XRegister {
    history: Vec<i32>,
    x: i32,
}

impl Debug for XRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (cycle, x) in self.history.iter().enumerate() {
            writeln!(f, "{cycle}: \t {x}")?;
        }

        Ok(())
    }
}

impl XRegister {
    fn new() -> XRegister {
        XRegister { history: Vec::new(), x: 1 }
    }

    fn read(&mut self, input: String) {
        let mut iter = input.lines();

        while let Some(line) = iter.next() {
            // Start cycle
            self.history.push(self.x);

            if line == "noop" {
                continue;
            } 

            let words: Vec<_> = line.split_whitespace().collect();
            
            let modifier: i32 = words[1].parse().unwrap();

            // Second cycle of addx
            self.history.push(self.x);

            self.x += modifier;
        }
    }

    fn get_signal_strength(&self) -> i32 {
        let mut sum: i32 = 0;
        
        for cycle in [20, 60, 100, 140, 180, 220] {
            // First cycle is at index 0 so 20th cycle is at index 19
            sum += cycle as i32 * self.history[cycle - 1];
        }

        sum
    }

    fn get_crt_image(&self) -> String {
        let mut result = String::new();

        for (cycle, x) in self.history.iter().enumerate() {
            if (x-1..=x+1).contains(&(cycle as i32 % 40)) {
                result.push('#');
            } else {
                result.push('.');
            }

            if cycle > 0  && (cycle + 1) % 40 == 0 {
                result.push('\n');
            }
        }

        result.trim_end().to_string()
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let mut register = XRegister::new();

    register.read(input);

    println!("Signal strength: {}", register.get_signal_strength());

    println!("CRT image:\n{}", register.get_crt_image());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input: String = fs::read_to_string("test_input.txt").unwrap();

        let mut register = XRegister::new();

        register.read(input);

        assert_eq!(register.get_signal_strength(), 13140);
    }
    
    #[test]
    fn test_input_image() {
        let input: String = fs::read_to_string("test_input.txt").unwrap();

        let mut register = XRegister::new();

        register.read(input);

        assert_eq!(
"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....".to_string(),
            register.get_crt_image()
        );
    }

    #[test]
    fn test_small_input() {
        let input = String::from("noop\naddx 3\naddx -5");

        let mut register = XRegister::new();

        register.read(input);

        assert_eq!(
            vec![1,1,1,4,4],
            register.history
        );
    }    

}
