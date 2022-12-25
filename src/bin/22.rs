use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Open,
    Wall,
    Nothing,
}

type Point = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up = 3,
    Down = 1,
    Left = 2,
    Right = 0,
}

#[derive(Debug, Clone, Copy)]
enum Step {
    Literal(usize),
    Left,
    Right,
}

trait Shape {}
struct Flat {}
struct Cube {}
impl Shape for Flat {}
impl Shape for Cube {}

#[derive(Debug)]
struct Grove<S: Shape> {
    board: Vec<Vec<Tile>>,
    x: usize,
    y: usize,
    direction: Direction,
    s: PhantomData<S>,
}

impl<S: Shape> Grove<S> {
    fn new(map_input: &str) -> Self {
        let mut board = Vec::new();
        let mut max = 0;
        for line in map_input.lines() {
            let mut temp = Vec::new();
            temp.push(Tile::Nothing);
            max = max.max(line.len());
            for tile in line.chars() {
                let tile = match tile {
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    ' ' => Tile::Nothing,
                    _ => unreachable!(),
                };
                temp.push(tile);
            }
            temp.extend(vec![Tile::Nothing; max - line.len()]);
            temp.push(Tile::Nothing);
            board.push(temp);
        }
        board.push(vec![Tile::Nothing; board[0].len()]);
        board.insert(0, vec![Tile::Nothing; board[0].len()]);
        let start = board[1].iter().position(|i| *i == Tile::Open).unwrap();
        Self {
            board,
            x: start,
            y: 1,
            direction: Direction::Right,
            s: PhantomData,
        }
    }

    fn get(&self, (x, y): Point) -> Tile {
        self.board[y][x]
    }

    fn open_point(&self, point: Point) -> bool {
        self.get(point) == Tile::Open
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl Grove<Flat> {
    fn advance(&mut self, step: Step) {
        match step {
            Step::Literal(steps) => {
                for _ in 0..steps {
                    let moved = self.move_forward();
                    if !moved {
                        break;
                    }
                }
            }
            Step::Left => self.turn_left(),
            Step::Right => self.turn_right(),
        }
    }

    fn move_forward(&mut self) -> bool {
        let mut new_point = match self.direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };

        if self.get(new_point) == Tile::Nothing {
            match self.direction {
                Direction::Up => {
                    new_point.1 = self.board.len()
                        - 1
                        - self
                            .board
                            .iter()
                            .rev()
                            .position(|row| row[self.x] != Tile::Nothing)
                            .unwrap();
                }
                Direction::Down => {
                    new_point.1 = self
                        .board
                        .iter()
                        .position(|row| row[self.x] != Tile::Nothing)
                        .unwrap();
                }
                Direction::Left => {
                    new_point.0 = self.board[self.y].len()
                        - 1
                        - self.board[self.y]
                            .iter()
                            .rev()
                            .position(|tile| *tile != Tile::Nothing)
                            .unwrap();
                }
                Direction::Right => {
                    new_point.0 = self.board[self.y]
                        .iter()
                        .position(|tile| *tile != Tile::Nothing)
                        .unwrap();
                }
            }
        }
        if self.open_point(new_point) {
            (self.x, self.y) = new_point;
            true
        } else {
            false
        }
    }
}

impl Grove<Cube> {
    fn advance(&mut self, step: Step) {
        match step {
            Step::Literal(steps) => {
                for _ in 0..steps {
                    let moved = self.move_forward();
                    if !moved {
                        break;
                    }
                }
            }
            Step::Left => self.turn_left(),
            Step::Right => self.turn_right(),
        }
    }

    fn move_forward(&mut self) -> bool {
        let mut new_point = match self.direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };
        let mut new_direction = self.direction;
        if self.get(new_point) == Tile::Nothing {
            match self.direction {
                Direction::Up => match self.x {
                    1..=50 => {
                        new_point = (51, self.x + 50);
                        new_direction = Direction::Right;
                    }
                    51..=100 => {
                        new_point = (1, self.x + 100);
                        new_direction = Direction::Right;
                    }
                    101..=150 => new_point = (self.x - 100, 200),
                    _ => unreachable!(),
                },
                Direction::Down => match self.x {
                    1..=50 => new_point = (self.x + 100, 1),
                    51..=100 => {
                        new_point = (50, self.x + 100);
                        new_direction = Direction::Left;
                    }
                    101..=150 => {
                        new_point = (100, self.x - 50);
                        new_direction = Direction::Left;
                    }
                    _ => unreachable!(),
                },
                Direction::Left => match self.y {
                    1..=50 => {
                        new_point = (1, 151 - self.y);
                        new_direction = Direction::Right;
                    }
                    51..=100 => {
                        new_point = (self.y - 50, 101);
                        new_direction = Direction::Down;
                    }
                    101..=150 => {
                        new_point = (51, 151 - self.y);
                        new_direction = Direction::Right;
                    }
                    151..=200 => {
                        new_point = (self.y - 100, 1);
                        new_direction = Direction::Down;
                    }
                    _ => unreachable!(),
                },
                Direction::Right => match self.y {
                    1..=50 => {
                        new_point = (100, 151 - self.y);
                        new_direction = Direction::Left;
                    }
                    51..=100 => {
                        new_point = (self.y + 50, 50);
                        new_direction = Direction::Up;
                    }
                    101..=150 => {
                        new_point = (150, 151 - self.y);
                        new_direction = Direction::Left;
                    }
                    151..=200 => {
                        new_point = (self.y - 100, 150);
                        new_direction = Direction::Up;
                    }
                    _ => unreachable!(),
                },
            }
        }
        if self.open_point(new_point) {
            (self.x, self.y) = new_point;
            self.direction = new_direction;
            true
        } else {
            false
        }
    }
}

fn parse_steps(input: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut buf = String::new();
    for c in input.chars() {
        match c {
            'R' => {
                steps.push(Step::Literal(buf.parse().unwrap()));
                buf.clear();
                steps.push(Step::Right);
            }
            'L' => {
                steps.push(Step::Literal(buf.parse().unwrap()));
                buf.clear();
                steps.push(Step::Left);
            }
            c => buf.push(c),
        }
    }
    // last step if exists
    if !buf.is_empty() {
        steps.push(Step::Literal(buf.parse().unwrap()));
    }
    steps
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map_input, steps_input) = input.split_once("\n\n").unwrap();
    let mut grove = Grove::<Flat>::new(map_input);
    for step in parse_steps(steps_input.trim()) {
        grove.advance(step);
    }
    let row = grove.y * 1000;
    let col = grove.x * 4;
    let facing = grove.direction as usize;

    Some((row + col + facing) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map_input, steps_input) = input.split_once("\n\n").unwrap();
    let mut grove = Grove::<Cube>::new(map_input);
    for step in parse_steps(steps_input.trim()) {
        grove.advance(step);
    }
    let row = grove.y * 1000;
    let col = grove.x * 4;
    let facing = grove.direction as usize;

    Some((row + col + facing) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
