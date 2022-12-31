use std::fs;

#[derive(Debug)]
struct Forest {
    rows: Vec<Vec<Tree>>
}

impl Forest {
    fn build(input: String) -> Forest {
        let mut forest = Forest { rows : Vec::new() };

        for line in input.lines() {
            let mut row = Vec::new();

            for byte in line.bytes() {
                row.push(Tree {
                    height: (byte - b'0') as i8,
                    visible: false
                });
            }

            forest.rows.push(row);
        }

        forest
    }
}

#[derive(Debug)]
struct Tree {
    height: i8,
    visible: bool
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let forest = Forest::build(input);
    // for row in forest.rows.iter_mut() {
    //     let mut min_height: i8 = -1;
    //     // From left to right
    //     for tree in row.iter_mut() {
    //         if tree.height > min_height {
    //             min_height = tree.height;
    //             tree.visible = true;
    //         }
    //     }
    //     min_height = -1;
    //     // From right to left
    //     for tree in row.iter_mut().rev() {
    //         if tree.height > min_height {
    //             min_height = tree.height;
    //             tree.visible = true;
    //         }
    //     }
    // } 

    // for column in 0..forest.rows[0].len() {
    //     let mut min_height: i8 = -1;
    //     // From top to bottom
    //     for row in forest.rows.iter_mut() {
    //         if row[column].height > min_height {
    //             min_height = row[column].height;
    //             row[column].visible = true;
    //         }
    //     }
    //     min_height = -1;
    //     // From bottom to top
    //     for row in forest.rows.iter_mut().rev() {
    //         if row[column].height > min_height {
    //             min_height = row[column].height;
    //             row[column].visible = true;
    //         }
    //     }
    // } 

    // let mut total = 0;

    // for row in forest.rows.iter() {
    //     // From right to left
    //     for tree in row.iter() {
    //        if tree.visible {
    //             total += 1;
    //        }
    //     }
    // } 

    // println!("{total}");
    let mut best_scenic_score = 0;

    for row in 0..forest.rows.len() {
        for col in 0..forest.rows[0].len() {
            let scenic_score = get_scenic_score(row, col, &forest);
            if  scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }
    
    println!("best scenic score: {best_scenic_score}");
}

fn get_scenic_score(row: usize, col: usize, forest: &Forest) -> usize {
    let tree_to_consider = &forest.rows[row][col];

    let mut viewing_dist_right = 0;
    for tree in &forest.rows[row][(col+1)..] {
        if tree.height < tree_to_consider.height {
            viewing_dist_right += 1;
        } else if tree.height >= tree_to_consider.height {
            viewing_dist_right += 1;
            break;
        }
    }

    let mut viewing_dist_left = 0;
    for tree in forest.rows[row][..col].iter().rev() {
        if tree.height < tree_to_consider.height {
            viewing_dist_left += 1;
        } else if tree.height >= tree_to_consider.height {
            viewing_dist_left += 1;
            break;
        }
    }

    let mut viewing_dist_below = 0;
    for row in &forest.rows[(row + 1)..] {
        if row[col].height < tree_to_consider.height {
            viewing_dist_below += 1;
        } else if row[col].height >= tree_to_consider.height {
            viewing_dist_below += 1;
            break;
        }
    }

    let mut viewing_dist_above = 0;
    for row in forest.rows[..row].iter().rev() {
        if row[col].height < tree_to_consider.height {
            viewing_dist_above += 1;
        } else if row[col].height >= tree_to_consider.height {
            viewing_dist_above += 1;
            break;
        }
    }

    viewing_dist_right * viewing_dist_left * viewing_dist_above * viewing_dist_below
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenic_score() {
        let input = fs::read_to_string("test_input.txt").unwrap();

        let forest = Forest::build(input);

        assert_eq!(4, get_scenic_score(1, 2, &forest));
        assert_eq!(8, get_scenic_score(3, 2, &forest));
    }
}