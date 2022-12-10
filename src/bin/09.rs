use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let moves = input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(' ').unwrap();
            let (dx, dy) = match a.as_bytes()[0] as char {
                'U' => (0, 1),
                'D' => (0, -1),
                'R' => (1, 0),
                'L' => (-1, 0),
                _ => unreachable!(),
            };
            (dx, dy, b.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut rope = vec![(0i32, 0i32); 2];
    let mut visited = HashSet::with_capacity(10000);
    for (dx, dy, len) in moves {
        for _ in 0..len {
            rope[0] = (rope[0].0 + dx, rope[0].1 + dy);
            for i in 1..rope.len() {
                let (dx, dy) = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                if dx.abs() > 1 || dy.abs() > 1 {
                    rope[i].0 += dx.signum();
                    rope[i].1 += dy.signum();
                }
            }
            visited.insert(rope[1]);
        }
    }
    return Some(visited.len().try_into().unwrap());
}

pub fn part_two(input: &str) -> Option<u32> {
    let moves = input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(' ').unwrap();
            let (dx, dy) = match a.as_bytes()[0] as char {
                'U' => (0, 1),
                'D' => (0, -1),
                'R' => (1, 0),
                'L' => (-1, 0),
                _ => unreachable!(),
            };
            (dx, dy, b.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut rope = vec![(0i32, 0i32); 10];
    let mut visited = HashSet::with_capacity(10000);
    for (dx, dy, len) in moves {
        for _ in 0..len {
            rope[0] = (rope[0].0 + dx, rope[0].1 + dy);
            for i in 1..rope.len() {
                let (dx, dy) = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                if dx.abs() > 1 || dy.abs() > 1 {
                    rope[i].0 += dx.signum();
                    rope[i].1 += dy.signum();
                }
            }
            visited.insert(rope[9]);
        }
    }
    return Some(visited.len().try_into().unwrap());
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
