pub fn part_one(input: &str) -> Option<u32> {
    let mut x = 1;
    let mut scores = vec![];

    let cycleStates = [20, 60, 100, 140, 180, 220];
    // Greed the split and add is going to turned into two steps
    for (i, inst) in input.split_whitespace().enumerate() {
        let cycle = i + 1;

        if cycleStates.contains(&cycle) {
            println!("{:?}", cycle);
            scores.push(x * cycle as i32);
        }

        if let Ok(num) = str::parse::<i32>(inst) {
            x += num;
        }
    }


    return Some(scores.iter().sum::<i32>().try_into().unwrap());
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut x = 1;
    let mut screen = String::new();

    for (i, inst) in input.split_whitespace().enumerate() {
        let cycle = (i % 40) as i32;

        if x - 1 <= cycle && cycle <= x + 1 {
            screen.push('#');
        } else {
            screen.push('.');
        }

        if let Ok(num) = str::parse::<i32>(inst) {
            x += num;
        }

        if (i + 1) % 40 == 0 {
            screen.push('\n');
        }
    }

    println!("{}", screen);

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
