use itertools::Itertools;
use std::collections::VecDeque;

use hashbrown::HashSet;


fn bfs(grid: &[Vec<u8>], start_node: (usize, usize), end_node: (usize, usize)) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut touched: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back((start_node, 0));
    touched.insert(start_node);
    while let Some(((x, y), length)) = queue.pop_front() {
        if (x, y) == end_node {
            return Some(length);
        }

        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (new_dx, new_dy) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            let Some(&value)  = grid.get(new_dy).and_then(|r| r.get(new_dx)) else {continue;};
            if grid[y][x] + 1 >= value && !touched.contains(&(new_dx, new_dy)) {
                touched.insert((new_dx, new_dy));
                queue.push_back(((new_dx, new_dy), length + 1));
            }
        }
    }
    None
}








pub fn part_one(input: &str) -> Option<u32> {
    let mut height_map = input
        .lines()
        .map(|line| line.as_bytes().iter().copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    

    // let mut touched = vec![vec![false; height_map[0].len()]; height_map.len()];

    // // Get current node
    // let mut x = 20;
    // let mut y = 0;
    let x_size = height_map[0].len();
    let y_size = height_map.len();
    println!("Grid size: {}x{}", x_size, y_size);
    let start_node = (0, 20);
    let end_node = (132, 20);

    // println!("{}", char::from(height_map[20][132]));

    height_map[start_node.1][start_node.0] = b'a';
    height_map[end_node.1][end_node.0] = b'z';

    return Some(bfs(&height_map, start_node, end_node).unwrap() as u32);

    
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut height_map = input
        .lines()
        .map(|line| line.as_bytes().iter().copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let end_node = (132, 20);

    height_map[20][0] = b'a';
    height_map[end_node.1][end_node.0] = b'z';

    let p2 = (0..height_map[0].len())
        .cartesian_product(0..height_map.len())
        .filter(|&(x, y)| height_map[y][x] == b'a')
        .filter_map(|start_node| bfs(&height_map, start_node, end_node))
        .min().unwrap() as u32;

    return Some(p2);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
