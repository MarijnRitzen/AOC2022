use std::{fs, collections::BinaryHeap, cmp::Ordering};

#[derive(PartialEq, Eq)]
struct Vertex {
    pos: (usize, usize, u8),
    cost: usize,
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Map {
    map: Vec<Vec<u8>>,
}

impl Map {
    fn parse(input: String) -> Map {
        let mut map = Vec::new();

        for input_line in input.lines() {
            let mut line = Vec::new();

            for byte in input_line.bytes() {
                line.push(byte);
            }

            map.push(line);
        }        

        Map {
            map
        }
    }

    fn shortest_path(&self, start_pos: (usize, usize, u8)) -> usize {
        // Find start node
        let pos = start_pos;
        let mut goal = (0, 0, 0);

        for (i, row) in self.map.iter().enumerate() {
            for (j, node) in row.iter().enumerate() {
                if *node == b'E' {
                    goal = (i, j, b'z');
                }
            }
        }

        let height = self.map.len();
        let width = self.map[0].len();
        let mut dist: Vec<Vec<_>> = (0..height).map(|_| (0..width).map(|_| usize::MAX).collect()).collect();

        let mut heap = BinaryHeap::new();

        // We're at `S`, with a zero cost
        dist[pos.0][pos.1] = 0;

        heap.push(Vertex { cost: 0, pos });

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(Vertex { cost, pos }) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            // println!("{pos:?}");
            if pos == goal { return cost; }

            // Important as we may have already found a better way
            if cost > dist[pos.0][pos.1] { continue; }

            for directions in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
                // Position of neighbour
                let i = pos.0 as i32 + directions.0;
                let j = pos.1 as i32 + directions.1;

                // Check bounds
                if i < 0 || i >= height as i32 || j < 0 || j >= width as i32 { continue; }

                let i = i as usize;
                let j = j as usize;

                let mut neighbour_height = self.map[i][j];

                if neighbour_height == b'E' {
                    neighbour_height = b'z';
                }

                // Check height
                if neighbour_height > pos.2 + 1 {
                    continue
                }

                // Relaxation
                if cost + 1 < dist[i][j] {
                    heap.push(Vertex { cost: cost + 1, pos: (i as usize, j as usize, neighbour_height) });

                    dist[i][j] = cost + 1;
                }
            }
        };

        // No path found
        usize::MAX
    }

    fn shortest_path_from_start(&self) -> usize {
        for (i, row) in self.map.iter().enumerate() {
            for (j, node) in row.iter().enumerate() {
                if *node == b'S' {
                    return self.shortest_path((i, j, b'a'));
                }
            }
        }
         
        usize::MAX
    }

    fn best_trail_length(&self) -> usize {
        let mut best = usize::MAX;

        for (i, row) in self.map.iter().enumerate() {
            for (j, node) in row.iter().enumerate() {
                if *node == b'S' || *node == b'a' {
                    let dist = self.shortest_path((i, j, b'a'));

                    if dist < best {
                        best = dist;
                    }
                }
            }
        }
         
        best
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let map = Map::parse(input);

    println!("Fewest steps to destination: {}", map.shortest_path_from_start());
    println!("Steps on best trail to destination: {}", map.best_trail_length());
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;


    #[test]
    fn test_input_part_one() {
        let input: String = fs::read_to_string("test_input.txt").unwrap();

        let map = Map::parse(input);
        
        let result = map.shortest_path_from_start();

        assert_eq!(result, 31);
    }

    #[test]
    fn test_input_part_two() {
        let input: String = fs::read_to_string("test_input.txt").unwrap();

        let map = Map::parse(input);
        
        let result = map.best_trail_length();

        assert_eq!(result, 29);
    }
}
