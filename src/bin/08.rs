pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .split('\n')
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();


    let mut top: Vec<Vec<u32>> = grid.iter().map(|line| vec![0; line.len()]).collect();
    let mut right: Vec<Vec<u32>> = grid.iter().map(|line| vec![0; line.len()]).collect();
    let mut bottom: Vec<Vec<u32>> = grid.iter().map(|line| vec![0; line.len()]).collect();
    let mut left: Vec<Vec<u32>> = grid.iter().map(|line| vec![0; line.len()]).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    println!("{:?} {:?}", rows, cols);

    for i in (0..(rows - 1)).rev() {
        for j in 0..cols {
            top[i][j] = top[i + 1][j].max(grid[i + 1][j]);
        }
    }

    for i in 1..rows {
        for j in 0..cols {
            bottom[i][j] = bottom[i - 1][j].max(grid[i - 1][j]);
        }
    }

    for j in (0..(cols - 1)).rev() {
        for i in 0..rows {
            right[i][j] = right[i][j + 1].max(grid[i][j + 1]);
        }
    }
    for j in 1..cols {
        for i in 0..rows {
            left[i][j] = left[i][j - 1].max(grid[i][j - 1]);
        }
    }

    let mut res = 0;
    for (row, line) in grid.iter().enumerate() {
        for col in 0..line.len() {
            if row == 0 || col == 0 || row == rows - 1 || col == cols - 1 {
                res += 1;
                continue;
            }
            let v = grid[row][col];
            if 
                top[row][col] == 0 ||
                right[row][col] == 0 ||
                bottom[row][col] == 0 ||
                left[row][col] == 0 {
                if v > 0 {
                    res += 1;
                }
                continue;
            }

            if top[row][col] < v {
                res += 1;
                continue;
            }
            if right[row][col] < v {
                res += 1;
                continue;
            }
            if bottom[row][col] < v {
                res += 1;
                continue;
            }
            if left[row][col] < v {
                res += 1;
                continue;
            }
        }
    }

    return Some(res);
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .split('\n')
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    
        let mut viewing_distances: Vec<u32> = Vec::new();

        for r in 1..99 - 1 {
            for c in 1..99 - 1 {
                let val = grid[r][c];
                let mut dist = 1;

                // UP
                dist *= match (r + 1..99).map(|y| grid[y][c]).position(|v| v >= val) {
                    Some(d) => d + 1,
                    None => 99 - r - 1,
                };
    
                // DOWN
                dist *= match (0..r).rev().map(|y| grid[y][c]).position(|v| v >= val) {
                    Some(d) => d + 1,
                    None => r,
                };
    
                // RIGHT
                dist *= match (c + 1..99).map(|x| grid[r][x]).position(|v| v >= val) {
                    Some(d) => d + 1,
                    None => 99 - c - 1,
                };
    
                // LEFT
                dist *= match (0..c).rev().map(|x| grid[r][x]).position(|v| v >= val) {
                    Some(d) => d + 1,
                    None => c,
                };
    
                viewing_distances.push(dist as u32);
            }
        }
    
        return viewing_distances.iter().max().copied();
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
