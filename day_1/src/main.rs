use std::fs;

fn main() {
    let string = fs::read_to_string("input.txt").unwrap();
    
    let calories = string.lines();


    let mut current_elf_sum: usize = 0;

    // Top three, best is first
    let mut top_three: [usize; 3] = [0,0,0];

    for calorie in calories {
        if let "" = calorie {
            // Update top three
            for place in 0..3 {
                if current_elf_sum > top_three[place] {
                    for later_place in (place + 1)..3 {
                        top_three[later_place] = top_three[later_place-1];
                    }
                    top_three[place] = current_elf_sum;
                    break;
                }
            }
            
            print!("Top three: {:?} \r", top_three);

            // Reset counter for next elf
            current_elf_sum = 0;
            continue;
        }
        
        current_elf_sum += calorie.parse::<usize>().unwrap();
    }
    println!("\n{}", top_three.iter().sum::<usize>());
}
