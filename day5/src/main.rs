use std::{fs, fmt::Debug};

struct Ship {
    stacks: Vec<Vec<char>>
}

impl Ship {
    fn build(input: &str) -> Ship {
        let mut stacks: Vec<Vec<char>> = Vec::new();

        let mut iter = input.lines().rev();

        let line = iter.next().unwrap();

        for _ in line.split_whitespace() {
            stacks.push(Vec::new());
        }

        for line in iter {
            for index in 0..stacks.len() {
                let character = line.chars().nth(1 + 4 * index).unwrap();

                if character != ' ' {
                    stacks[index].push(character);
                }
            }
        }

        Ship { stacks }
    }

    fn apply_instruction(&mut self, count: usize, from: usize, to: usize) {
        let length_from = self.stacks[from - 1].len();

        let mut to_move = self.stacks[from - 1].split_off(length_from - count);

        self.stacks[to - 1].append(&mut to_move);

        // for _ in 0..count {
        //     self.move_container(from, to);
        // }
    }

    fn move_container(&mut self, from: usize, to: usize) {
        let container = self.stacks[from - 1].pop().unwrap();

        self.stacks[to - 1].push(container);
    }

    fn print_top_crates(&self) {
        for stack in &self.stacks {
            print!("{:}", stack.last().unwrap());
        } 
        println!();
    }
}

impl Debug for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_height = self.stacks.iter().fold(0, |accum, stack| if stack.len() > accum {stack.len()} else {accum});

        for row in 1..(max_height + 1) {
            for stack in &self.stacks {
                if let Some(name) = stack.get(max_height - row) {
                    write!(f, " [{name}]")?;
                } else {
                    write!(f, "    ")?;
                }
            }
            write!(f, "\n")?;
        }
        for (index, _) in (&self.stacks).iter().enumerate() {
            write!(f, "  {} ", index + 1)?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();


    let space_index = input.find("\n\n").unwrap();

    let ship_input = &input[..=space_index];

    let mut ship = Ship::build(ship_input);

    let instructions = &input[(space_index + 2)..];
    let instructions = instructions.lines();

    println!("START:\n{:?}", ship);
    for (index, instruction) in instructions.enumerate() {
        // Move a from b to c
        if let [_, count, _, from, _, to] = instruction.split_whitespace().collect::<Vec<_>>()[..] {
            ship.apply_instruction(count.parse().unwrap(), from.parse().unwrap(), to.parse().unwrap());
        }
        // println!("after {instruction}:\n{:?}", ship);
    }

    ship.print_top_crates();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_instruction() {
        let mut ship = Ship {
            stacks: vec![vec!['A', 'B'], vec![]]
        };

        ship.apply_instruction(2, 1, 2);

        assert_eq!(ship.stacks, vec![vec![], vec!['A', 'B']]);
    }

    #[test]
    fn test_split_off() {
        let mut vector = vec!['A', 'B', 'C'];
        
        assert_eq!(vec!['B', 'C'], vector.split_off(3 - 2));
    }
}