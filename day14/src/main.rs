use std::{fmt::Debug, thread::sleep, fs};

static PART_2: bool = true;

type CaveColumn = Vec<u8>;
type CaveLayout = Vec<CaveColumn>;

struct Cave {
    layout: CaveLayout,
    sand_source: usize, // Only x coordinate necessary
    width: (usize, usize),
    depth: usize
}

impl Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_depth_digits = self.depth.to_string().len();        

        for pos in 0..3 {
            for _ in 0..max_depth_digits {
                write!(f, " ")?
            }
            for index in self.width.0..=self.width.1 {
                if index % 2 == 0 {
                    write!(f, "{}", format!("{index:0>3}").chars().nth(pos).unwrap())?
                } else {
                    write!(f, " ")?
                }
            }
            writeln!(f, "")?;
        }
        for depth in 0..=self.depth {
            write!(f, "{depth:>width$}", width = max_depth_digits)?;
            for index in 0..(self.width.1-self.width.0 + 1) {
                if depth == 0 && index == self.sand_source {
                    write!(f, "+")?;
                } else {
                    write!(f, "{}", self.layout[index][depth] as char)?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl Cave {
    fn get_restful_sand_units(&mut self) -> usize { 
        let mut result = 0;

        while let Ok(()) = self.place_sand() {
            result += 1;
        }

        result
    }

    fn place_sand(&mut self) -> Result<(), ()> {
        let mut position = (self.sand_source, 0);
        
        // Check if there is already sand at the source
        if PART_2 && self.layout[position.0][position.1] == b'o' {
            return Err(());
        }

        self.layout[position.0][position.1] = b'o';

        while let Ok(new_position) = self.move_sand_once(position) {
            // println!("{self:?}");
            // sleep(std::time::Duration::from_millis(100));
            if new_position == position { 
                return  Ok(());
            }

            position = new_position;
        }

        Err(())

            // let depth = self.drop_sand_vertically(position)?;

    }

    fn move_sand_once(&mut self, from: (usize, usize)) -> Result<(usize, usize), ()> {
        // Reset previous position to air
        self.layout[from.0][from.1] = b'.';

        // First check if the sand will fall off the world, 
        // then if it can drop down, or left, or right
        if from.0 == 0 || from.0 == self.layout.len() ||from.1 == self.depth {
            Err(())
        } else if self.layout[from.0][from.1 + 1] == b'.' {
            self.layout[from.0][from.1 + 1] = b'o';

            Ok((from.0, from.1 + 1))
        } else if self.layout[from.0 - 1][from.1 + 1] == b'.'  {
            self.layout[from.0 - 1][from.1 + 1] = b'o';

            Ok((from.0 - 1, from.1 + 1))
        } else if self.layout[from.0 + 1][from.1 + 1] == b'.' {
            self.layout[from.0 + 1][from.1 + 1] = b'o';

            Ok((from.0 + 1, from.1 + 1))
        } else {
            self.layout[from.0][from.1] = b'o';

            Ok(from)
        }
    }
}

impl From<String> for Cave {
    fn from(input: String) -> Self {
        let mut width = (usize::MAX, 0);        
        let mut depth = 0;        

        // First we want to know how big to make the CaveLayout
        for line in input.lines() {
            for coordinates in line.split(" -> ") {
                let (x, y) = coordinates.split_once(",").unwrap();
                let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
            
                if x < width.0 {
                    width.0 = x;
                }
                if x > width.1 {
                    width.1 = x;
                }
                if y > depth {
                    depth = y;
                }
            }
        }

        if PART_2 {
            width = (500 - depth - 2, 500 + depth + 2);
            depth += 2;
        }
        // Initialize CaveLayout
        let mut cave_layout: CaveLayout = CaveLayout::new();

        for _ in 0..width.1 - width.0 + 1 {
            let col = (0..=depth).map(|_| b'.').collect();

            cave_layout.push(col);
        }

        // Fill walls
        for line in input.lines() {
            if PART_2 {
                for x in 0..width.1 - width.0 + 1 {
                    cave_layout[x][depth] = b'#';
                }
            }
            for window in line
                .split(" -> ")
                .map(|coords| coords.split_once(",").unwrap())
                .map(|(x,y)| (x.parse::<usize>().unwrap() - width.0, y.parse::<usize>().unwrap()))
                .collect::<Vec<_>>()
                .windows(2) {
                if let [(x_s, y_s), (x_t, y_t)] = window {
                    for x in std::cmp::min(*x_s, *x_t)..=std::cmp::max(*x_s, *x_t) {
                        for y in std::cmp::min(*y_s, *y_t)..=std::cmp::max(*y_s, *y_t) {
                            cave_layout[x][y] = b'#';
                        }
                    }
                } else {
                    unreachable!()
                }

            }
        }

        Cave {
            layout: cave_layout,
            sand_source: 500 - width.0,
            width,
            depth
        } 
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut cave = Cave::from(input);

    println!("Result: {}", cave.get_restful_sand_units());
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{Cave, PART_2};


    #[test]
    fn test_input() {
        let input = fs::read_to_string("test_input.txt").unwrap();

        let mut cave = Cave::from(input);

        if PART_2 {
            assert_eq!(93, cave.get_restful_sand_units());
        } else {
            assert_eq!(24, cave.get_restful_sand_units());
        }
    }
}
