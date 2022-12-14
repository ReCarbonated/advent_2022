pub fn part_one(input: &str) -> Option<u32> {
    let coords = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|string| {
                    // println!("{}", string);
                    let (x_str, y_str) = string.split_once(",").unwrap();
                    // println!("{}, {}", x_str, y_str);
                    let x = x_str.parse::<u32>().unwrap();
                    let y = y_str.parse::<u32>().unwrap();
                    (x, y)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let x_max = coords
        .iter()
        .map(|row| row.iter().map(|item| item.0).max().unwrap())
        .max()
        .unwrap();
    let y_max = coords
        .iter()
        .map(|row| row.iter().map(|item| item.1).max().unwrap())
        .max()
        .unwrap();

    let x_min = coords
        .iter()
        .map(|row| row.iter().map(|item| item.0).min().unwrap())
        .min()
        .unwrap();
    let y_min = coords
        .iter()
        .map(|row| row.iter().map(|item| item.1).min().unwrap())
        .min()
        .unwrap();

    println!("{}, {}", x_max, y_max);
    println!("{}, {}", x_min, y_min);

    let mut cave = vec![vec!["."; 540]; 170];

    for walls in coords.iter() {
        let mut wall = walls.iter().peekable();
        while let Some(&first) = wall.next() {
            while let Some(&&second) = wall.peek() {
                // println!("Points: {}x{}, {}x{}", first.0, first.1, second.0, second.1);
                match (first, second) {
                    ((x, _), (y, _)) if (x == y) => {
                        let distance: i32 = first.1 as i32 - second.1 as i32;
                        for idx in 0..distance.abs() + 1 {
                            if distance.is_negative() {
                                // println!("Creating wall at: {},{}", first.0 as usize, first.1 as usize + idx as usize);
                                cave[first.1 as usize + idx as usize][first.0 as usize] = "#"
                            } else {
                                // println!("Creating wall at: {},{}", first.0 as usize, first.1 as usize - idx as usize);
                                cave[first.1 as usize - idx as usize][first.0 as usize] = "#"
                            }
                        }
                    }
                    ((_, x), (_, y)) if (x == y) => {
                        let distance: i32 = first.0 as i32 - second.0 as i32;
                        for idx in 0..distance.abs() + 1 {
                            if distance.is_negative() {
                                // println!("Creating wall at: {},{}", first.0 as usize + idx as usize, first.1 as usize);
                                cave[first.1 as usize][first.0 as usize + idx as usize] = "#"
                            } else {
                                // println!("Creating wall at: {},{}", first.0 as usize - idx as usize, first.1 as usize);
                                cave[first.1 as usize][first.0 as usize - idx as usize] = "#"
                            }
                        }
                    }
                    (_, _) => {}
                }
                break;
            }
        }
    }

    let mut sand = 0;
    let mut full = false;

    while !full {
        let mut x = 500;
        let mut y = 0;
        loop {
            if (y + 1) >= cave.len() {
                full = true;
                break;
            }
            if cave[y + 1][x] == "." {
                y += 1;
            } else if cave[y + 1][x - 1] == "." {
                x -= 1;
                y += 1;
            } else if cave[y + 1][x + 1] == "." {
                x += 1;
                y += 1;
            } else {
                cave[y][x] = "o";
                break;
            }
        }
        sand += 1;
    }

    Some(sand - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let coords = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|string| {
                    // println!("{}", string);
                    let (x_str, y_str) = string.split_once(",").unwrap();
                    // println!("{}, {}", x_str, y_str);
                    let x = x_str.parse::<u32>().unwrap();
                    let y = y_str.parse::<u32>().unwrap();
                    (x, y)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let x_max = coords
        .iter()
        .map(|row| row.iter().map(|item| item.0).max().unwrap())
        .max()
        .unwrap();
    let y_max = coords
        .iter()
        .map(|row| row.iter().map(|item| item.1).max().unwrap())
        .max()
        .unwrap();

    let x_min = coords
        .iter()
        .map(|row| row.iter().map(|item| item.0).min().unwrap())
        .min()
        .unwrap();
    let y_min = coords
        .iter()
        .map(|row| row.iter().map(|item| item.1).min().unwrap())
        .min()
        .unwrap();

    println!("{}, {}", x_max, y_max);
    println!("{}, {}", x_min, y_min);

    let mut cave = vec![vec!["."; 900]; 170];

    for walls in coords.iter() {
        let mut wall = walls.iter().peekable();
        while let Some(&first) = wall.next() {
            while let Some(&&second) = wall.peek() {
                // println!("Points: {}x{}, {}x{}", first.0, first.1, second.0, second.1);
                match (first, second) {
                    ((x, _), (y, _)) if (x == y) => {
                        let distance: i32 = first.1 as i32 - second.1 as i32;
                        for idx in 0..distance.abs() + 1 {
                            if distance.is_negative() {
                                // println!("Creating wall at: {},{}", first.0 as usize, first.1 as usize + idx as usize);
                                cave[first.1 as usize + idx as usize][first.0 as usize] = "#"
                            } else {
                                // println!("Creating wall at: {},{}", first.0 as usize, first.1 as usize - idx as usize);
                                cave[first.1 as usize - idx as usize][first.0 as usize] = "#"
                            }
                        }
                    }
                    ((_, x), (_, y)) if (x == y) => {
                        let distance: i32 = first.0 as i32 - second.0 as i32;
                        for idx in 0..distance.abs() + 1 {
                            if distance.is_negative() {
                                // println!("Creating wall at: {},{}", first.0 as usize + idx as usize, first.1 as usize);
                                cave[first.1 as usize][first.0 as usize + idx as usize] = "#"
                            } else {
                                // println!("Creating wall at: {},{}", first.0 as usize - idx as usize, first.1 as usize);
                                cave[first.1 as usize][first.0 as usize - idx as usize] = "#"
                            }
                        }
                    }
                    (_, _) => {}
                }
                break;
            }
        }
    }

    for x in 0..cave[0].len() {
        cave[y_max as usize + 2][x] = "#";
    }

    let mut sand = 0;
    let mut full = false;

    while !full {
        let mut x = 500;
        let mut y = 0;
        loop {
            if cave[y + 1][x] == "." {
                y += 1;
            } else if cave[y + 1][x - 1] == "." {
                x -= 1;
                y += 1;
            } else if cave[y + 1][x + 1] == "." {
                x += 1;
                y += 1;
            } else {
                cave[y][x] = "o";
                if x == 500 && y == 0 {
                    full = true;
                    break;
                }
                break;
            }
        }
        sand += 1;
    }

    Some(sand)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), None);
    }
}
