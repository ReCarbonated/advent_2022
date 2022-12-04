pub fn part_one(input: &str) -> Option<u32> {
    let output: u32 = input
        .split_terminator(&['-', ',', '\n'][..])
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .chunks(4)
        .map(|x| -> [u32; 4] {x.try_into().unwrap()})
        .fold(0, |a, [s1, e1, s2, e2]| {
            a + u32::from(s2>= s1 && e2 <= e1 || s1 >= s2 && e1 <= e2)
        });
    return Some(output);
}

pub fn part_two(input: &str) -> Option<u32> {
    let output: u32 = input
        .split_terminator(&['-', ',', '\n'][..])
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .chunks(4)
        .map(|x| -> [u32; 4] {x.try_into().unwrap()})
        .fold(0, |a, [s1, e1, s2, e2]| {
            a + u32::from(s1 >= s2 && s1 <= e2 || s2 >= s1 && s2 <= e1)
        });
    return Some(output);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
