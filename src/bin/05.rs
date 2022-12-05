pub fn part_one(input: &str) -> Option<u32> {
    let mut stacks: Vec<Vec<_>> = vec![Vec::new(); 9];

    for line in input.lines() {
        // it splits per 4 offset by 1
        if line.contains("[") {
            let items: Vec<char> = line.chars()
                .enumerate()
                .filter(|&(i, _)| i % 4 == 1)
                .map(|(_, v)| v)
                .collect();

            for (idx, chr) in items.into_iter().enumerate() {
                if chr.is_alphabetic() {
                    stacks[idx].insert(0, chr);
                }
            }
        } else if line.contains("move") {
            let moves = line
                .split_whitespace()
                .filter_map(|s| {
                    match s.parse::<usize>() {
                        Ok(num) => Some(num),
                        Err(_) => None
                    }
                })
                .collect::<Vec<usize>>();

            // We should be able to do our steps now
            let stack = &mut stacks[moves[1] - 1];
            let mut items = stack.split_off(stack.len() - moves[0]);
            items.reverse();
            let stack = &mut stacks[moves[2] - 1];
            stack.extend(items);
        }
    }

    for mut stack in stacks {
        print!("{}", stack.pop().unwrap());
    }
    println!();
    return None;
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut stacks: Vec<Vec<_>> = vec![Vec::new(); 9];

    for line in input.lines() {
        // it splits per 4 offset by 1
        if line.contains("[") {
            let items: Vec<char> = line.chars()
                .enumerate()
                .filter(|&(i, _)| i % 4 == 1)
                .map(|(_, v)| v)
                .collect();

            for (idx, chr) in items.into_iter().enumerate() {
                if chr.is_alphabetic() {
                    stacks[idx].insert(0, chr);
                }
            }
        } else if line.contains("move") {
            let moves = line
                .split_whitespace()
                .filter_map(|s| {
                    match s.parse::<usize>() {
                        Ok(num) => Some(num),
                        Err(_) => None
                    }
                })
                .collect::<Vec<usize>>();

            // We should be able to do our steps now
            let stack = &mut stacks[moves[1] - 1];
            let mut items = stack.split_off(stack.len() - moves[0]);
            let stack = &mut stacks[moves[2] - 1];
            stack.extend(items);
        }
    }

    for mut stack in stacks {
        print!("{}", stack.pop().unwrap());
    }
    println!();
    return None;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
