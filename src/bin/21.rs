use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Clone)]
enum Operation {
    Num(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn search(
    monkies: &HashMap<String, Operation>,
    cache: &mut HashMap<String, i64>,
    name: &str,
) -> i64 {
    if let Some(&v) = cache.get(name) {
        return v;
    }
    let v = match &monkies[name] {
        Operation::Num(i) => *i,
        Operation::Add(m1, m2) => search(monkies, cache, &m1) + search(monkies, cache, &m2),
        Operation::Sub(m1, m2) => search(monkies, cache, &m1) - search(monkies, cache, &m2),
        Operation::Mul(m1, m2) => search(monkies, cache, &m1) * search(monkies, cache, &m2),
        Operation::Div(m1, m2) => search(monkies, cache, &m1) / search(monkies, cache, &m2),
    };
    cache.insert(name.to_string(), v);
    v
}

pub fn part_one(input: &str) -> Option<u32> {
    let monkies = input
        .lines()
        .map(|l| {
            let (name, res) = l.split_once(": ").unwrap();
            let op = match res.split(" ").collect_tuple() {
                Some((monkey1, "+", monkey2)) => {
                    Operation::Add(monkey1.to_string(), monkey2.to_string())
                }
                Some((monkey1, "-", monkey2)) => {
                    Operation::Sub(monkey1.to_string(), monkey2.to_string())
                }
                Some((monkey1, "*", monkey2)) => {
                    Operation::Mul(monkey1.to_string(), monkey2.to_string())
                }
                Some((monkey1, "/", monkey2)) => {
                    Operation::Div(monkey1.to_string(), monkey2.to_string())
                }
                _ => Operation::Num(res.parse().unwrap()),
            };
            (name.to_string(), op)
        })
        .collect::<HashMap<_, _>>();
    let mut cache = HashMap::new();
    let p1 = search(&monkies, &mut cache, "root");

    dbg!(p1);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let monkies = input
        .lines()
        .map(|l| {
            let (name, res) = l.split_once(": ").unwrap();
            let op = match res.split(" ").collect_tuple() {
                Some((monkey1, "+", monkey2)) => {
                    Operation::Add(monkey1.to_string(), monkey2.to_string())
                }
                Some((monkey1, "-", monkey2)) => {
                    Operation::Sub(monkey1.to_string(), monkey2.to_string())
                }
                Some((monkey1, "*", monkey2)) => {
                    Operation::Mul(monkey1.to_string(), monkey2.to_string())
                }
                Some((monkey1, "/", monkey2)) => {
                    Operation::Div(monkey1.to_string(), monkey2.to_string())
                }
                _ => Operation::Num(res.parse().unwrap()),
            };
            (name.to_string(), op)
        })
        .collect::<HashMap<_, _>>();
    let mut x = 3087390115000;
    loop {
        let mut cloned_monkies = monkies.clone();
        cloned_monkies.insert("humn".to_string(), Operation::Num(x));
        let mut cache = HashMap::new();
        if search(&cloned_monkies, &mut cache, "fflg") == 52282191702834 {
            println!("val: {}", x);
            break;
        } else if search(&cloned_monkies, &mut cache, "fflg") < 52282191702834 {
            dbg!(x, search(&cloned_monkies, &mut cache, "fflg"));
            break;
        } else {
            dbg!(x, search(&cloned_monkies, &mut cache, "fflg"));
            x += 1;
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), None);
    }
}
