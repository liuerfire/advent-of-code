use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn walk(
    coords: &HashMap<(i32, i32), char>,
    current: (i32, i32),
    direction: Direction,
) -> Vec<(i32, i32, Direction)> {
    let mut ret = vec![];
    let ch = coords.get(&(current.0, current.1)).unwrap();
    match (ch, direction) {
        ('.', Direction::Up)
        | ('|', Direction::Up)
        | ('\\', Direction::Left)
        | ('/', Direction::Right) => {
            let next = (current.0, current.1 - 1);
            ret.push((next.0, next.1, Direction::Up));
        }
        ('.', Direction::Down)
        | ('|', Direction::Down)
        | ('\\', Direction::Right)
        | ('/', Direction::Left) => {
            let next = (current.0, current.1 + 1);
            ret.push((next.0, next.1, Direction::Down));
        }
        ('.', Direction::Left)
        | ('-', Direction::Left)
        | ('/', Direction::Down)
        | ('\\', Direction::Up) => {
            let next = (current.0 - 1, current.1);
            ret.push((next.0, next.1, Direction::Left));
        }
        ('.', Direction::Right)
        | ('-', Direction::Right)
        | ('/', Direction::Up)
        | ('\\', Direction::Down) => {
            let next = (current.0 + 1, current.1);
            ret.push((next.0, next.1, Direction::Right));
        }
        ('-', Direction::Up) | ('-', Direction::Down) => {
            let left = (current.0 - 1, current.1);
            ret.push((left.0, left.1, Direction::Left));

            let right = (current.0 + 1, current.1);
            ret.push((right.0, right.1, Direction::Right));
        }
        ('|', Direction::Left) | ('|', Direction::Right) => {
            let up = (current.0, current.1 - 1);
            ret.push((up.0, up.1, Direction::Up));

            let down = (current.0, current.1 + 1);
            ret.push((down.0, down.1, Direction::Down));
        }
        _ => unreachable!(),
    }
    ret
}

fn one(input: &str) -> usize {
    let mut coords: HashMap<(i32, i32), char> = HashMap::default();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            coords.insert((x as i32, y as i32), c);
        }
    }

    energize_tiles(&coords, 0, 0, Direction::Right)
}

fn energize_tiles(
    coords: &HashMap<(i32, i32), char>,
    x: i32,
    y: i32,
    direction: Direction,
) -> usize {
    let mut visited: Vec<(i32, i32, Direction)> = vec![];
    let mut paths = vec![((x, y), direction)];
    loop {
        if paths.is_empty() {
            break;
        }
        let (current, direction) = paths.pop().unwrap();
        if coords.get(&(current.0, current.1)).is_none() {
            continue;
        }
        if visited.contains(&(current.0, current.1, direction.clone())) {
            continue;
        }
        visited.push((current.0, current.1, direction.clone()));
        for p in walk(&coords, current, direction) {
            paths.push(((p.0, p.1), p.2.clone()));
        }
    }
    let mut seen: HashSet<(i32, i32)> = HashSet::default();
    for i in visited {
        seen.insert((i.0, i.1));
    }
    seen.len()
}

fn two(input: &str) -> usize {
    let mut coords: HashMap<(i32, i32), char> = HashMap::default();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            coords.insert((x as i32, y as i32), c);
        }
    }

    let lines = input.split_terminator('\n').collect::<Vec<&str>>();
    let rows = lines.len();
    let cols = lines[0].chars().collect::<Vec<char>>().len();

    let mut num = 0;
    for i in 0..rows {
        num = max(num, energize_tiles(&coords, i as i32, 0, Direction::Up));
        num = max(num, energize_tiles(&coords, i as i32, 0, Direction::Down));
    }
    for i in 0..cols {
        num = max(num, energize_tiles(&coords, 0, i as i32, Direction::Left));
        num = max(num, energize_tiles(&coords, 0, i as i32, Direction::Right));
    }
    num
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";
        assert_eq!(46, one(input.trim()));
        assert_eq!(51, two(input.trim()));
    }

    #[test]
    fn test_answer() {
        println!(
            "{}",
            one(read_to_string("data/2023/day16.txt").unwrap().trim())
        );
        println!(
            "{}",
            two(read_to_string("data/2023/day16.txt").unwrap().trim())
        );
    }
}
