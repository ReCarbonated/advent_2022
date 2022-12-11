pub fn part_one(input: &str) -> Option<u32> {
    pub struct Monkey {
        pub items: Vec<u64>,
        pub test: u64,
        pub operation: (u8, u64),
        pub target_1: usize,
        pub target_2: usize,
        pub inspect_count: u32,
    }

    let mut monkey_collection = input.split("\n\n").map(|monkey_input| {
        let mut monkey = Monkey {
            items: Vec::with_capacity(32),
            test: 0,
            operation: (0, 0),
            target_1: 0,
            target_2: 0,
            inspect_count: 0
        };
        for (idx, line) in monkey_input.lines().enumerate().skip(1) {
            match idx {
                1 => {
                    monkey.items.extend(line.split_once(": ").unwrap().1.split(", ").map(|n| n.parse::<u64>().expect(n)));
                }
                2 => {
                    if line.contains("+") {
                        monkey.operation = (0, line.split_once("+ ").unwrap().1.parse::<u64>().unwrap());
                    }
                    if line.contains("* old") {
                        monkey.operation = (2, 0);
                    }
                    else if line.contains("*") {
                        monkey.operation = (1, line.split_once("* ").unwrap().1.parse::<u64>().unwrap());
                    }
                }
                3 => {
                    monkey.test = line.split_once("by ").unwrap().1.parse::<u64>().unwrap();
                }
                4 => {
                    monkey.target_1 = line.split_once("monkey ").unwrap().1.parse::<usize>().unwrap();
                }
                5 => {
                    monkey.target_2 = line.split_once("monkey ").unwrap().1.parse::<usize>().unwrap();
                }
                _ => {}
            }
        }
        monkey
    }).collect::<Vec<Monkey>>();

    //simulation
    let mut passes: Vec<(usize, u64)> = Vec::with_capacity(32);
    for _rounds in 0..20 {
        println!("Round {}", _rounds);
        for (idx, monkey) in monkey_collection.iter().enumerate() {
            println!("{} {} {:?}", idx, monkey.inspect_count, monkey.items);
        }
        println!("\n");

        for idx in 0..monkey_collection.len() {
            let monkey = monkey_collection.get_mut(idx).unwrap();
            passes.clear();
            for item in monkey.items.iter_mut() {
                
                monkey.inspect_count += 1;
                match monkey.operation.0 {
                    0 => {
                        *item += monkey.operation.1;
                    }
                    1 => {
                        *item *= monkey.operation.1;
                    }
                    _ => {
                        *item *= *item;
                    }
                }
                *item /= 3;
                if *item % monkey.test == 0 {
                    // println!("To: Monkey {}, {:?}", monkey.target_1, item);
                    passes.push((monkey.target_1, *item));
                } else {
                    // println!("To: Monkey {}, {:?}", monkey.target_2, item);
                    passes.push((monkey.target_2, *item));
                }
            }
            monkey.items.clear();
            for (target, item) in &passes {
                // println!("To: Monkey {}, {:?}", target, item);
                monkey_collection[*target].items.push(*item);
            }
        }
        
    }
    let mut counts = monkey_collection
        .iter()
        .map(|m| m.inspect_count)
        .collect::<Vec<u32>>();
    counts.sort_unstable();
    println!("{:?}", counts);
    Some(counts.iter().rev().take(2).product::<u32>())
}

pub fn part_two(input: &str) -> Option<u64> {
    pub struct Monkey {
        pub items: Vec<u64>,
        pub test: u64,
        pub operation: (u8, u64),
        pub target_1: usize,
        pub target_2: usize,
        pub inspect_count: u64,
    }


    let mut monkey_collection = input.split("\n\n").map(|monkey_input| {
        let mut monkey = Monkey {
            items: Vec::with_capacity(32),
            test: 0,
            operation: (0, 0),
            target_1: 0,
            target_2: 0,
            inspect_count: 0
        };
        for (idx, line) in monkey_input.lines().enumerate().skip(1) {
            match idx {
                1 => {
                    monkey.items.extend(line.split_once(": ").unwrap().1.split(", ").map(|n| n.parse::<u64>().expect(n)));
                }
                2 => {
                    if line.contains("+") {
                        monkey.operation = (0, line.split_once("+ ").unwrap().1.parse::<u64>().unwrap());
                    }
                    if line.contains("* old") {
                        monkey.operation = (2, 0);
                    }
                    else if line.contains("*") {
                        monkey.operation = (1, line.split_once("* ").unwrap().1.parse::<u64>().unwrap());
                    }
                }
                3 => {
                    monkey.test = line.split_once("by ").unwrap().1.parse::<u64>().unwrap();
                }
                4 => {
                    monkey.target_1 = line.split_once("monkey ").unwrap().1.parse::<usize>().unwrap();
                }
                5 => {
                    monkey.target_2 = line.split_once("monkey ").unwrap().1.parse::<usize>().unwrap();
                }
                _ => {}
            }
        }
        monkey
    }).collect::<Vec<Monkey>>();

    //simulation
    let mut passes: Vec<(usize, u64)> = Vec::with_capacity(32);
    let lcm = monkey_collection.iter().map(|m| m.test).product::<u64>();
    for _rounds in 0..10000 {
        for idx in 0..monkey_collection.len() {
            let monkey = monkey_collection.get_mut(idx).unwrap();
            passes.clear();
            for item in monkey.items.iter_mut() {
                monkey.inspect_count += 1;
                match monkey.operation.0 {
                    0 => {
                        *item = item.wrapping_add(monkey.operation.1) % lcm;
                    }
                    1 => {
                        *item = item.wrapping_mul(monkey.operation.1) % lcm;
                    }
                    _ => {
                        *item = item.wrapping_mul(*item) % lcm;
                    }
                }
                if *item % monkey.test == 0 {
                    // println!("To: Monkey {}, {:?}", monkey.target_1, item);
                    passes.push((monkey.target_1, *item));
                } else {
                    // println!("To: Monkey {}, {:?}", monkey.target_2, item);
                    passes.push((monkey.target_2, *item));
                }
            }
            monkey.items.clear();
            for (target, item) in &passes {
                // println!("To: Monkey {}, {:?}", target, item);
                monkey_collection[*target].items.push(*item);
            }
        }
        
    }
    let mut counts = monkey_collection
        .iter()
        .map(|m| m.inspect_count)
        .collect::<Vec<u64>>();
    counts.sort_unstable();
    println!("{:?}", counts.iter().rev().take(2).product::<u64>());
    Some(counts.iter().rev().take(2).product::<u64>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
