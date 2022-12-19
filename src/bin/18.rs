use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Hash)]
struct Cube {
    x: i64,
    y: i64,
    z: i64,
    idx: usize,
}

impl Cube {
    fn new(x: i64, y: i64, z: i64, idx: usize) -> Cube {
        Cube {
            x: x,
            y: y,
            z: z,
            idx: 0,
        }
    }

    fn from_string(line: &str) -> Cube {
        let vals = line.split(',').collect::<Vec<_>>();
        Cube {
            x: vals[0].parse().unwrap(),
            y: vals[1].parse().unwrap(),
            z: vals[2].parse().unwrap(),
            idx: 0,
        }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube {
            x: 0,
            y: 0,
            z: 0,
            idx: 0,
        }
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Cube {}

impl Iterator for Cube {
    type Item = Cube;

    fn next(&mut self) -> Option<Cube> {
        let result = match self.idx {
            0 => Some(Cube::new(self.x - 1, self.y, self.z, 1)),
            1 => Some(Cube::new(self.x + 1, self.y, self.z, 2)),
            2 => Some(Cube::new(self.x, self.y - 1, self.z, 3)),
            3 => Some(Cube::new(self.x, self.y + 1, self.z, 4)),
            4 => Some(Cube::new(self.x, self.y, self.z - 1, 5)),
            5 => Some(Cube::new(self.x, self.y, self.z + 1, 6)),
            _ => None,
        };
        self.idx += 1;
        result
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut surface_area = 0;
    let cubes = input
        .lines()
        .map(|line| Cube::from_string(line))
        .collect::<HashSet<_>>();

    for &cube in cubes.iter() {
        for adjacent_cube in cube {
            if !cubes.contains(&adjacent_cube) {
                surface_area += 1;
            }
        }
    }
    Some(surface_area)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut surface_area = 0;
    let mut cubes = input
        .lines()
        .map(|line| Cube::from_string(line))
        .collect::<HashSet<_>>();

    for &cube in cubes.iter() {
        for adjacent_cube in cube {
            if !cubes.contains(&adjacent_cube) {
                surface_area += 1;
            }
        }
    }

    let min_x = cubes.iter().map(|cube| cube.x).min().unwrap() - 2;
    let max_x = cubes.iter().map(|cube| cube.x).max().unwrap() + 2;
    let min_y = cubes.iter().map(|cube| cube.y).min().unwrap() - 2;
    let max_y = cubes.iter().map(|cube| cube.y).max().unwrap() + 2;
    let min_z = cubes.iter().map(|cube| cube.z).min().unwrap() - 2;
    let max_z = cubes.iter().map(|cube| cube.z).max().unwrap() + 2;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            cubes.insert(Cube::new(x, y, min_z, 0));
            cubes.insert(Cube::new(x, y, max_z, 0));
        }
    }
    for x in min_x..=max_x {
        for z in min_z..=max_z {
            cubes.insert(Cube::new(x, min_y, z, 0));
            cubes.insert(Cube::new(x, max_y, z, 0));
        }
    }
    for y in min_y..=max_y {
        for z in min_z..=max_z {
            cubes.insert(Cube::new(min_x, y, z, 0));
            cubes.insert(Cube::new(max_x, y, z, 0));
        }
    }

    let start = Cube::new(min_x + 1, min_y + 1, min_z + 1, 0);
    let mut queue = vec![start];
    while let Some(cube) = queue.pop() {
        if cubes.insert(cube) {
            for adjacent_cube in cube {
                if !cubes.contains(&adjacent_cube) {
                    queue.push(adjacent_cube);
                }
            }
        }
    }

    let mut new_surface_area = 0;
    for &cube in cubes.iter() {
        for adjacent_cube in cube {
            if !cubes.contains(&adjacent_cube) {
                new_surface_area += 1;
            }
        }
    }

    let expected_new_external_surface_area = {
        2 * (max_x - min_x + 1) * (max_y - min_y + 1) +
        2 * (max_x - min_x + 1) * (max_z - min_z + 1) +
        2 * (max_y - min_y + 1) * (max_z - min_z + 1)
    };
    let internal_surface_area = new_surface_area - expected_new_external_surface_area;
    let external_surface_area = surface_area - internal_surface_area;


    println!("{}", external_surface_area);
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), None);
    }
}
