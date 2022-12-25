fn mixer(nums: Vec<i64>, count: usize, key: i64) -> i64 {
    let nums = nums.iter().map(|x| x * key).collect::<Vec<_>>();
    let length = nums.len();
    let mut ans = (0..length).collect::<Vec<_>>();
    for _ in 0..count {
        for (i, &x) in nums.iter().enumerate() {
            let pos = ans.iter().position(|&y| y == i).unwrap();
            ans.remove(pos);
            let new_i = (pos as i64 + x).rem_euclid((length - 1) as i64) as usize;
            ans.insert(new_i, i);
        }
    }
    let zero_idx = nums.iter().position(|&i| i == 0).unwrap();
    let rebase = ans.iter().position(|&i| i == zero_idx).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| nums[ans[(rebase + i) % length]])
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let encrypted = input
        .lines()
        .map(|l| l.to_string().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    dbg!(mixer(encrypted, 1, 1));
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let encrypted = input
    .lines()
    .map(|l| l.to_string().parse::<i64>().unwrap())
    .collect::<Vec<i64>>();

    dbg!(mixer(encrypted, 10, 811589153));
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
