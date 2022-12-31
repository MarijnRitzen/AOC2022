use std::{collections::HashSet, fs};

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn distance(&self, other: &Position) -> u32 {
        self.x.abs_diff(other.x) as u32 + self.y.abs_diff(other.y) as u32
    }
}

struct Sensor {
    pos: Position,
    neigherest_beacon: Beacon,
}

struct Beacon {
    pos: Position,
}

struct Subterrain {
    sensors: Vec<Sensor>,
}

impl Subterrain {
    fn impossible_places(&self, row: i32) -> u32 {
        let mut impossible: HashSet<i32> = HashSet::new();

        // First add all places that fall under the sensors coverage
        for sensor in &self.sensors {
            let dist_to_sensor = row.abs_diff(sensor.pos.y);
            let dist_sensor_closes_beacon = sensor.pos.distance(&sensor.neigherest_beacon.pos);

            // if sensor is relevant
            if dist_to_sensor <= dist_sensor_closes_beacon {

                let surplus = (dist_sensor_closes_beacon - dist_to_sensor) as i32;

                // Add all impossible places to the set
                for place in (sensor.pos.x - surplus)..=(sensor.pos.x + surplus) {
                    impossible.insert(place);
                }
            }
        }

        // Now remove all places that contain a beacon
        for sensor in &self.sensors {
            let beacon_pos = &sensor.neigherest_beacon.pos;

            if beacon_pos.y == row {
                impossible.remove(&beacon_pos.x);
            }
        }

        impossible.len() as u32
    }
    
    fn tuning_frequency(&self) -> i32 {
        for x in 0..(if cfg!(test) { 20 } else { 4_000_000 }) {
            'place: for y in 0..(if cfg!(test) { 20 } else { 4_000_000 }) {
                for sensor in &self.sensors {
                    let dist_to_sensor = Position { x, y }.distance(&sensor.pos);
                    let dist_sensor_closes_beacon = sensor.pos.distance(&sensor.neigherest_beacon.pos);

                    if dist_to_sensor <= dist_sensor_closes_beacon {
                        continue 'place;
                    }
                }

                return x * 4000000 + y;
            }

            if x % 100 == 0 {
                println!("Progress: {}%", x as f64 / 4_000_000f64 * 100f64);
            }
        }
        0
    }
}

impl From<String> for Subterrain {
    fn from(input: String) -> Self {
        let mut sensors = Vec::new();

        for line in input.lines() {
            let positions: Vec<_> = line
                .split_whitespace()
                .map(|w| w.trim_end_matches(|c| c == ',' || c == ':'))
                .filter_map(|w| {
                    w.split_once("=")
                        .map(|t| t.1)
                        .map(|n| n.parse::<i32>().unwrap())
                })
                .collect();

            sensors.push(Sensor {
                pos: Position {
                    x: positions[0],
                    y: positions[1],
                },
                neigherest_beacon: Beacon {
                    pos: Position {
                        x: positions[2],
                        y: positions[3],
                    },
                },
            })
        }

        Subterrain { sensors }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let subterrain = Subterrain::from(input);

    // println!("result part 1: {}", subterrain.impossible_places(2000000));
    println!("result part 2: {}", subterrain.tuning_frequency());
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::Subterrain;

    #[test]
    fn test_input() {
        let input = fs::read_to_string("test_input.txt").unwrap();

        let subterrain = Subterrain::from(input);

        assert_eq!(26, subterrain.impossible_places(10));
        assert_eq!(56000011, subterrain.tuning_frequency());
    }
}
