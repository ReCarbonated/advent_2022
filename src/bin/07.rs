use hashbrown::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    let mut stack:Vec<&str> = vec![];
    let mut map: HashMap<Vec<&str>, Vec<(&str, u32)>> = HashMap::<_, Vec<(&str, u32)>>::default();

    // Create a peekable view into list
    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() { 
        match line {
            "$ ls" => {
                // Create/Enter new entry in the mapping
                let v = map.entry(stack.clone()).or_default();
                while let Some(line) = lines.peek() {
                    if line.starts_with("$") {
                        break;
                    }
                    if line.starts_with("dir") {
                        lines.next();
                        continue;
                    }

                    let (l, r) = lines.next().unwrap().split_once(" ").unwrap();
                    v.push((r, l.parse().unwrap()));
                }
            }
            "$ cd .." => {stack.pop().unwrap();},
            "$ cd /" => stack.clear(),
            _ if line.starts_with("$ cd ") => stack.push(&line[5..]),
            _ => unreachable!()
        }
    }


    let v: Vec<(&Vec<&str>, u32)>= map
        .iter()
        .map(|(s, v)| 
            (
                s, 
                v.iter().map(|&(_, i)| i).sum()
            )
        )
        .collect();

    let mut storage = vec![];

    for (s, _) in &v {
        let mut sum = 0;
        for (s2, i) in &v {
            if s2.len() >= s.len() && s.iter().eq(&s2[..s.len()]) {
                sum += i;
            }
        }

        storage.push((s, sum));
    }

    return Some(storage.iter().map(|(_, i)| i).filter(|&&i| i < 100_000).sum());
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut stack:Vec<&str> = vec![];
    let mut map: HashMap<Vec<&str>, Vec<(&str, u32)>> = HashMap::<_, Vec<(&str, u32)>>::default();

    // Create a peekable view into list
    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() { 
        match line {
            "$ ls" => {
                // Create/Enter new entry in the mapping
                let v = map.entry(stack.clone()).or_default();
                while let Some(line) = lines.peek() {
                    if line.starts_with("$") {
                        break;
                    }
                    if line.starts_with("dir") {
                        lines.next();
                        continue;
                    }

                    let (l, r) = lines.next().unwrap().split_once(" ").unwrap();
                    v.push((r, l.parse().unwrap()));
                }
            }
            "$ cd .." => {stack.pop().unwrap();},
            "$ cd /" => stack.clear(),
            _ if line.starts_with("$ cd ") => stack.push(&line[5..]),
            _ => unreachable!()
        }
    }


    let v: Vec<(&Vec<&str>, u32)>= map
        .iter()
        .map(|(s, v)| 
            (
                s, 
                v.iter().map(|&(_, i)| i).sum()
            )
        )
        .collect();

    let mut storage = vec![];

    for (s, _) in &v {
        let mut sum = 0;
        for (s2, i) in &v {
            if s2.len() >= s.len() && s.iter().eq(&s2[..s.len()]) {
                sum += i;
            }
        }

        storage.push((s, sum));
    }

    let root = storage.iter().find(|(s, _)| s.is_empty()).unwrap().1;
    let needed = 30_000_000 - (70_000_000 - root);
    
    return Some(*storage.iter().map(|(_, i)| i).filter(|&&i| i >= needed).min().unwrap());

}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
