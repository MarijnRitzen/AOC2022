use std::{fs, collections::VecDeque, cell::RefCell, fmt::Debug};

static ROUND_2: bool = true;

struct Monkey {
    items: VecDeque<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    true_monkey: usize,
    false_monkey: usize,
    inspection_count: usize
}

impl Monkey {
    fn inspect_and_throw(&mut self) -> Option<(usize, usize)> {
        let item = self.items.pop_front()?;

        self.inspection_count += 1;

        let after_inspect_and_divide =  if ROUND_2 {
            (self.op)(item)
        } else {
            (self.op)(item) / 3
        };


        if (self.test)(after_inspect_and_divide) {
            Some((self.true_monkey, after_inspect_and_divide))
        } else {
            Some((self.false_monkey, after_inspect_and_divide))
        }
    }
}

struct Jungle {
    monkeys: Vec<RefCell<Monkey>>,
    main_divisor: usize
}

impl Debug for Jungle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, monkey) in self.monkeys.iter().enumerate() {
            writeln!(f, "Monkey {index} inspected item {} times", monkey.borrow().inspection_count)?;
        }

        Ok(())
    }
}

impl Jungle {
    fn parse(input: String) -> Jungle {
        let blocks = input.split("\n\n");

        let mut monkeys = Vec::new();
        let mut main_divisor = 1;

        for block in blocks {
            let lines: Vec<&str> = block.lines().collect();
            
            let items: VecDeque<usize> = lines[1].split_once(':').unwrap().1.split(',').map(|s| s.trim().parse().unwrap()).collect();

            let op = Self::parse_operation(lines[2].split_once('=').unwrap().1);

            let divisor: usize = lines[3].split_once("by").unwrap().1.trim().parse().unwrap();

            let test = Box::new(move |lvl: usize| lvl % divisor == 0);

            let true_monkey: usize = lines[4].split_once("monkey").unwrap().1.trim().parse().unwrap();

            let false_monkey: usize = lines[5].split_once("monkey").unwrap().1.trim().parse().unwrap();

            monkeys.push(RefCell::new(Monkey {
                items,
                op,
                test,
                true_monkey,
                false_monkey,
                inspection_count: 0
            }));

            main_divisor *= divisor;
        }

        Jungle {
            monkeys, 
            main_divisor
        }
    }

    fn parse_operation(operation: &str) -> Box<dyn Fn(usize) -> usize> {
        let parts: Vec<_> = operation.split_whitespace().collect();

        if parts[1] == "*" {
            match parts[2] {
                "old" => Box::new(|old| old * old),
                constant => {
                    let constant: usize = constant.parse().unwrap();

                    Box::new(move |old| old * constant)
                }
            }
        } else {
            match parts[2] {
                "old" => Box::new(|old| old + old),
                constant => {
                    let constant: usize = constant.parse().unwrap();

                    Box::new(move |old| old + constant)
                }
            }
        }
    }   

    fn start_mayham(&mut self) {
        let max_round = if ROUND_2 { 10_000 } else { 20 };

        for _ in 0..max_round {
            for monkey in &self.monkeys {
                while let Some((recipient, lvl)) = monkey.borrow_mut().inspect_and_throw() {
                    let thrown_item = if ROUND_2 {
                        lvl % self.main_divisor
                    } else {
                        lvl
                    };

                    self.monkeys[recipient].borrow_mut().items.push_back(thrown_item);
                }
            }

        }
    }    
    
    fn get_result(&mut self) -> usize {
        self.monkeys.sort_by(|a, b| a.borrow().inspection_count.cmp(&b.borrow().inspection_count));

        self.monkeys.reverse();

        let first_inspections = self.monkeys[0].borrow().inspection_count;
        let second_inspections = self.monkeys[1].borrow().inspection_count;

        first_inspections * second_inspections
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut jungle = Jungle::parse(input);

    jungle.start_mayham();

    println!("{}", jungle.get_result());
}

#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn test_closure_generation() {
        assert_eq!(Jungle::parse_operation("old * old")(3), 9);
        assert_eq!(Jungle::parse_operation("old + old")(3), 6);
        assert_eq!(Jungle::parse_operation("old + 5")(1), 6);
        assert_eq!(Jungle::parse_operation("old * 3")(1), 3);
    }

    #[test]
    fn test_inspect_and_throw() {
        let mut monkey = Monkey {
            items: [3].into(),
            op: Box::new(|old| old * 2),
            test: Box::new(|lvl| lvl % 3 == 0),
            true_monkey: 1,
            false_monkey: 2,
            inspection_count: 0
        };

        assert_eq!(monkey.inspect_and_throw(), if ROUND_2 { Some((1, 6)) } else { Some((2, 2)) });
    }

    #[test]
    fn test_input() {
        let input = fs::read_to_string("test_input.txt").unwrap();

        let mut jungle = Jungle::parse(input);

        jungle.start_mayham();

        assert_eq!(jungle.get_result(), if ROUND_2 { 2713310158 } else { 10605 } );
    }

    #[test]
    fn test_equivalence() {
        let input_lvl = 13113919;
        let modulo = 6;
        let addition = 31;
        assert_eq!(input_lvl + addition % modulo == 0, (input_lvl % modulo) + addition % modulo == 0);
    }
}