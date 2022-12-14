static ROCKS: [[[u8; 4]; 4]; 5] = [
    [*b"####", *b"    ", *b"    ", *b"    "],
    [*b" #  ", *b"### ", *b" #  ", *b"    "],
    [*b"### ", *b"  # ", *b"  # ", *b"    "],
    [*b"#   ", *b"#   ", *b"#   ", *b"#   "],
    [*b"##  ", *b"##  ", *b"    ", *b"    "],
];

fn collision(rock: &[[u8; 4]; 4], x: usize, y: usize, board: &Vec<[u8; 9]>) -> bool {
    for dy in 0..4 {
        for dx in 0..4 {
            if rock[dy][dx] == b'#' && board[y + dy][x + dx] == b'#' {
                return true;
            }
        }
    }
    false
}

fn find_repeating<T: Eq>(range: &[T]) -> Option<usize> {
    let len = range.len();

    for sub_len in 1.max(len / 3)..len / 2 {
        if range[len - sub_len * 2..len - sub_len].eq(&range[len - sub_len..]) {
            return Some(sub_len);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let input: Vec<isize> = input
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => 0,
        })
        .collect();

    let mut rocks = ROCKS.iter().enumerate().cycle();
    let mut wind = input.iter().copied().enumerate().cycle();

    let mut board: Vec<[u8; 9]> = vec![[b'#'; 9]; 1];
    let mut max = 0;

    let mut i = 0;
    while i < 2022 {
        i += 1;
        let rock = rocks.next().unwrap().1;
        board.resize(max + 8, *b"#       #");
        let start = max + 4;
        let mut y = start;
        let mut x = 3;
        let mut wind_dir;
        loop {
            wind_dir = wind.next().unwrap().1;
            let new_x = x + wind_dir;
            if !collision(rock, new_x as usize, y, &board) {
                x = new_x;
            }
            let new_y = y - 1;
            if collision(rock, x as usize, new_y, &board) {
                break;
            } else {
                y = new_y;
            }
        }

        for dy in 0..4 {
            for dx in 0..4 {
                if rock[dy][dx] == b'#' {
                    board[y + dy][x as usize + dx] = b'#';
                    max = max.max(y + dy);
                }
            }
        }
    }

    Some(max as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input: Vec<isize> = input
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => 0,
        })
        .collect();

    let mut rocks = ROCKS.iter().enumerate().cycle();
    let mut wind = input.iter().copied().enumerate().cycle();

    let mut board: Vec<[u8; 9]> = vec![[b'#'; 9]; 1];
    let mut max = 0;

    let mut moves = Vec::new();
    let mut heights = Vec::new();

    let mut to_add = None;

    let mut i = 0;
    while i < 1000000000000 {
        i += 1;
        let (kind, rock) = rocks.next().unwrap();
        board.resize(max + 8, *b"#       #");
        let start = max + 4;
        let mut y = start;
        let mut x = 3;
        let (mut wind_idx, mut wind_dir);
        loop {
            (wind_idx, wind_dir) = wind.next().unwrap();
            let new_x = x + wind_dir;
            if !collision(rock, new_x as usize, y, &board) {
                x = new_x;
            }
            let new_y = y - 1;
            if collision(rock, x as usize, new_y, &board) {
                break;
            } else {
                y = new_y;
            }
        }

        for dy in 0..4 {
            for dx in 0..4 {
                if rock[dy][dx] == b'#' {
                    board[y + dy][x as usize + dx] = b'#';
                    max = max.max(y + dy);
                }
            }
        }

        if to_add.is_none() {
            moves.push((kind, wind_idx, x, start - y));
            if let Some(len) = find_repeating(&moves) {
                let rocks_left = 1000000000000 - i;
                let height_diff = max - heights[heights.len() - len];
                let batches = rocks_left / len;
                to_add = Some(height_diff * batches);
                i += batches * len;
            }
            heights.push(max);
        }
    }

    println!("{}", (max + to_add.unwrap_or(0)).to_string());
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
