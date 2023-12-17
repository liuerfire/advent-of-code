use std::{
    collections::{HashMap, HashSet},
    str::Lines,
};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn walk(&self, c: char) -> Option<Direction> {
        match (self, c) {
            (Direction::Up, '|') => Some(Direction::Up),
            (Direction::Up, '7') => Some(Direction::Left),
            (Direction::Up, 'F') => Some(Direction::Right),
            (Direction::Down, '|') => Some(Direction::Down),
            (Direction::Down, 'J') => Some(Direction::Left),
            (Direction::Down, 'L') => Some(Direction::Right),
            (Direction::Left, '-') => Some(Direction::Left),
            (Direction::Left, 'L') => Some(Direction::Up),
            (Direction::Left, 'F') => Some(Direction::Down),
            (Direction::Right, '-') => Some(Direction::Right),
            (Direction::Right, 'J') => Some(Direction::Up),
            (Direction::Right, '7') => Some(Direction::Down),
            // got "."
            _ => None,
        }
    }
}

fn next(coord: (usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (coord.0, coord.1 - 1),
        Direction::Down => (coord.0, coord.1 + 1),
        Direction::Right => (coord.0 + 1, coord.1),
        Direction::Left => (coord.0 - 1, coord.1),
    }
}

fn one(lines: Lines) -> u32 {
    let mut start: (usize, usize) = (0, 0);
    let mut coords: HashMap<(usize, usize), char> = HashMap::default();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            coords.insert((x, y), c);
            if c == 'S' {
                start = (x, y);
            }
        }
    }
    let mut paths = vec![];
    let mut seen = HashSet::from([start]);
    for d in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let cand = next(start, &d);
        let cand_c = coords.get(&cand).unwrap();
        if let Some(next_d) = d.walk(*cand_c) {
            paths.push((1, next_d, cand));
            seen.insert(cand);
        }
    }
    loop {
        let mut new_paths = vec![];
        for (n, d, pos) in paths {
            let cand = next(pos, &d);
            let cand_c = coords.get(&cand).unwrap();
            if let Some(next_d) = d.walk(*cand_c) {
                if seen.get(&cand).is_some() {
                    return n + 1;
                }
                new_paths.push((n + 1, next_d, cand));
                seen.insert(cand);
            }
        }
        paths = new_paths;
    }
}

fn two(lines: Lines) -> u32 {
    let mut start: (usize, usize) = (0, 0);
    let mut coords: HashMap<(usize, usize), char> = HashMap::default();
    for (y, line) in lines.clone().enumerate() {
        for (x, c) in line.chars().enumerate() {
            coords.insert((x, y), c);
            if c == 'S' {
                start = (x, y);
            }
        }
    }
    let mut paths = vec![];
    let mut seen = HashSet::from([start]);
    for d in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let cand = next(start, &d);
        let cand_c = coords.get(&cand).unwrap();
        if let Some(next_d) = d.walk(*cand_c) {
            paths.push((1, next_d, cand));
            seen.insert(cand);
        }
    }

    let mut finished = false;
    loop {
        let mut new_paths = vec![];
        for (n, d, pos) in paths {
            let cand = next(pos, &d);
            let cand_c = coords.get(&cand).unwrap();
            if let Some(next_d) = d.walk(*cand_c) {
                if seen.get(&cand).is_some() {
                    finished = true;
                    break;
                }
                new_paths.push((n + 1, next_d, cand));
                seen.insert(cand);
            }
        }
        if finished {
            break;
        }
        paths = new_paths;
    }

    let mut s = 0;
    for (y, line) in lines.clone().enumerate() {
        for x in 0..line.chars().collect::<Vec<char>>().len() {
            if seen.get(&(x, y)).is_some() {
                continue;
            }
            let intersections = get_intersections(&lines, &seen, x, y);
            if intersections % 2 == 1 {
                s += 1
            }
        }
    }
    s
}

// https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm
fn get_intersections(lines: &Lines, seen: &HashSet<(usize, usize)>, x: usize, y: usize) -> u32 {
    let line: Vec<char> = lines.clone().collect::<Vec<&str>>()[y].chars().collect();
    let mut count = 0;
    for (i, c) in line.iter().enumerate().take(x) {
        if seen.get(&(i, y)).is_none() {
            continue;
        }
        if c == &'J' || c == &'L' || c == &'|' {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
.....
.S-7.
.|.|.
.L-J.
.....
";
        assert_eq!(4, one(input.trim().lines()));
    }

    #[test]
    fn test_answer() {
        println!(
            "{}",
            one(read_to_string("data/2023/day10.txt").unwrap().lines())
        );
        println!(
            "{}",
            two(read_to_string("data/2023/day10.txt").unwrap().lines())
        );
    }
}
