use std::{fs, collections::HashSet, fmt::Debug, io::{Stdout, stdout, Write}, time::Duration, num};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point(i32, i32);

struct MotionController {
    visited: HashSet<Point>,
    knots: Vec<Point>,
    stdout: Stdout,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Debug for MotionController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let max_x = self.knots.iter().max_by(|x,y| x.0.cmp(&y.0)).map(|max| max.0 as usize).unwrap();
        // let max_y = self.knots.iter().max_by(|x,y| x.1.cmp(&y.1)).map(|max| max.1 as usize).unwrap();
        // let min_x = self.knots.iter().min_by(|x,y| x.0.cmp(&y.0)).map(|min| min.0 as usize).unwrap();
        // let min_y = self.knots.iter().min_by(|x,y| x.1.cmp(&y.1)).map(|min| min.1 as usize).unwrap();

        for y in (-10..10).rev() {
            for x in -10..10 {
                if let Some(knot) = self.knots.iter().position(|k| k.0 == x as i32 && k.1 == y as i32) {
                    write!(f, "{}", { if knot > 0 {knot.to_string()} else {"H".to_string()} })?;
                } else if x == 0 && y == 0{
                    write!(f, "s")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft
}

impl From<&str> for Dir {
    fn from(dir: &str) -> Self {
        match dir {
            "U" => Self::Up,
            "D" => Self::Down,
            "R" => Self::Right,
            "L" => Self::Left,
            _ => panic!("Unexpected dir: {dir}")
        }
    }
}

impl MotionController {
    fn new(nr_knots: i32) -> MotionController {
        let mut knots = Vec::new();        

        for _ in 0..nr_knots {
            knots.push(Point(0,0));
        }

        MotionController {
            visited: HashSet::from([Point(0,0)]),
            stdout: stdout(),
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
            knots 
        }
    }

    fn run(input: String, nr_knots: i32) -> MotionController {
        let mut controller = MotionController::new(nr_knots);

        for (dir, steps) in input.lines().map(|l| l.split_whitespace().collect()).map(|a: Vec<_>| (a[0].into(), a[1].parse().unwrap())) {
            // println!("== {dir:?} {steps} ==");

            controller.handle_all_knots(dir, steps);
        }

        controller
    }

    fn handle_all_knots(&mut self, dir: Dir, steps: i32) {
        for _ in 0..steps  {
            // First just move the head            
            match dir {
                Dir::Up => self.knots[0].1 += 1,
                Dir::Down => self.knots[0].1 -= 1,
                Dir::Right => self.knots[0].0 += 1,
                Dir::Left => self.knots[0].0 -= 1,
                _ => unreachable!()
            }

            // Update min/max x/y for cool console output
            self.max_x = std::cmp::max(self.max_x, self.knots[0].0);
            self.min_x = std::cmp::min(self.min_x, self.knots[0].0);
            self.max_y = std::cmp::max(self.max_y, self.knots[0].1);
            self.min_y = std::cmp::min(self.min_y, self.knots[0].1);

            // Then for the knots after the head, let them move depending on the knot before them
            for knot in 1..self.knots.len() {
                // Determine the direction that the next knot will take
                let diff_x = self.knots[knot-1].0 - self.knots[knot].0;
                let diff_y = self.knots[knot-1].1 - self.knots[knot].1;
                let dir = match (diff_x, diff_y) {
                    (x, y) if x == 2 && y == 2 => Dir::UpRight,
                    (x, y) if x == 2 && y == -2 => Dir::DownRight,
                    (x, y) if x == -2 && y == -2 => Dir::DownLeft,
                    (x, y) if x == -2 && y == 2 => Dir::UpLeft,
                    (_, y) if y == 2 => Dir::Up,
                    (_, y) if y == -2 => Dir::Down,
                    (x, _) if x == 2 => Dir::Right,
                    (x, _) if x == -2 => Dir::Left,
                    _ => Dir::Up
                };

                self.handle_single_knot(knot as usize, dir);
            }

            // Add position of last knot to visited set
            self.visited.insert(*self.knots.last().unwrap());

            // print!("{esc}c", esc = 27 as char);
            // print!("{self:?}\n\n");

            // self.stdout.flush().unwrap();
            // std::thread::sleep(Duration::from_millis(50));
        }
    }
    
    fn handle_single_knot(&mut self, knot: usize, dir: Dir) {
        let head = self.knots[knot - 1];
        let tail = self.knots[knot];
        let hx = head.0;
        let hy = head.1;
        let mut tx = tail.0;
        let mut ty = tail.1;

        match dir {
            Dir::UpRight => {
                self.knots[knot] = Point(tx+1, ty+1);
                return;
            }
            Dir::DownRight => {
                self.knots[knot] = Point(tx+1, ty-1);
                return;
            }
            Dir::DownLeft => {
                self.knots[knot] = Point(tx-1, ty-1);
                return;
            }
            Dir::UpLeft => {
                self.knots[knot] = Point(tx-1, ty+1);
                return;
            }
            _ => ()
        };
        // Check if the tail needs to follow 
        let diff_first_axis = match dir {
            Dir::Up | Dir::Down => (hy - ty).abs(),
            Dir::Left | Dir::Right => (hx - tx).abs(),
            _ => unreachable!()
        };
        
        if diff_first_axis > 1 {
            // Check if the tail also needs to move in second axis

            let diff_second_axis = match dir {
                Dir::Up | Dir::Down => hx - tx,
                Dir::Left | Dir::Right => hy - ty,
                _ => unreachable!()
            };

            if diff_second_axis != 0 {
                match dir {
                    Dir::Up | Dir::Down => tx += diff_second_axis,
                    Dir::Left | Dir::Right => ty += diff_second_axis,
                    _ => unreachable!()
                }
            } 
        
            match dir {
                Dir::Up => ty += 1,
                Dir::Down => ty -= 1,
                Dir::Right => tx += 1,
                Dir::Left => tx -= 1,
                _ => unreachable!()
            }
        }

        let new_point = Point(tx, ty);

        self.knots[knot] = new_point;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let controller = MotionController::run(input, 10);

    println!("Number of visited places: {}", controller.visited.len());
    println!("min_x={}, max_x={}, min_y={}, max_y={}", controller.min_x, controller.max_x, controller.min_y, controller.max_y);

}

#[cfg(test)]
mod tests {
    use crate::MotionController;
    use super::*;

    #[test]
    fn up_right() {
        let mut controller = MotionController::new(2);

        controller.knots[0] = Point(1,1);

        controller.handle_all_knots(Dir::Up, 1);

        assert_eq!(controller.knots[1], Point(1,1));
    }

    #[test]
    fn up_left() {
        let mut controller = MotionController::new(2);

        controller.knots[0] = Point(-1,1);

        controller.handle_all_knots(Dir::Up, 1);

        assert_eq!(controller.knots[1], Point(-1,1));
    }    
    
    #[test]
    fn up_middle() {
        let mut controller = MotionController::new(2);

        controller.knots[0] = Point(0,1);

        controller.handle_all_knots(Dir::Up, 1);

        assert_eq!(controller.knots[1], Point(0,1));
    }    
    
    #[test]
    fn right_up() {
        let mut controller = MotionController::new(2);

        controller.knots[0] = Point(1,1);

        controller.handle_all_knots(Dir::Right, 1);

        assert_eq!(controller.knots[1], Point(1,1));
    }

    #[test]
    fn right_down() {
        let mut controller = MotionController::new(2);

        controller.knots[0] = Point(1,-1);

        controller.handle_all_knots(Dir::Right, 1);

        assert_eq!(controller.knots[1], Point(1,-1));
    }    
    
    #[test]
    fn right_middle() {
        let mut controller = MotionController::new(2);

        controller.knots[0] = Point(1,0);

        controller.handle_all_knots(Dir::Right, 1);

        assert_eq!(controller.knots[1], Point(1,0));
    }    
    
    #[test]
    fn small_input_2_knots() {
        let input = fs::read_to_string("test_input_small.txt").unwrap();

        let controller = MotionController::run(input, 2);

        assert_eq!(controller.visited.len(), 13);
    }
    
    #[test]
    fn small_input_10_knots() {
        let input = fs::read_to_string("test_input_small.txt").unwrap();

        let controller = MotionController::run(input, 10);

        assert_eq!(controller.visited.len(), 1);
    }
    
    #[test]
    fn large_input_10_knots() {
        let input = fs::read_to_string("test_input_large.txt").unwrap();

        let controller = MotionController::run(input, 10);

        assert_eq!(controller.visited.len(), 36);
    }
}
