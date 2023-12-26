use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Copy)]
struct Pos {
    heat_loss: u32,
    x: i32,
    y: i32,
    delta_x: i32,
    delta_y: i32,
    step: u32,
}

impl Pos {
    fn new(heat_loss: u32, x: i32, y: i32, delta_x: i32, delta_y: i32, step: u32) -> Self {
        Self {
            heat_loss,
            x,
            y,
            delta_x,
            delta_y,
            step,
        }
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heat_loss.cmp(&other.heat_loss)
    }
}

impl Eq for Pos {}

impl Hash for Pos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.x);
        state.write_i32(self.y);
        state.write_i32(self.delta_x);
        state.write_i32(self.delta_y);
        state.write_u32(self.step);
    }
}

fn one(input: &str) -> u32 {
    let mut coords: HashMap<(i32, i32), u32> = HashMap::default();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            coords.insert((x as i32, y as i32), c.to_digit(10).unwrap());
        }
    }
    let lines = input.split_terminator('\n').collect::<Vec<&str>>();
    let rows = lines.len() as i32;
    let cols = lines[0].chars().collect::<Vec<char>>().len() as i32;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(Pos::new(0, 0, 0, 0, 0, 0)));

    let mut seen: HashSet<Pos> = HashSet::default();
    while !heap.is_empty() {
        let current = heap.pop().unwrap().0;
        if current.x == rows - 1 && current.y == cols - 1 {
            return current.heat_loss;
        }

        if seen.contains(&current) {
            continue;
        }
        seen.insert(current);

        if current.step < 3 && (current.delta_x, current.delta_y) != (0, 0) {
            let next_x = current.x + current.delta_x;
            let next_y = current.y + current.delta_y;
            if coords.get(&(next_x, next_y)).is_some() {
                heap.push(Reverse(Pos::new(
                    current.heat_loss + *coords.get(&(next_x, next_y)).unwrap(),
                    next_x,
                    next_y,
                    current.delta_x,
                    current.delta_y,
                    current.step + 1,
                )));
            }
        }

        for delta in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if delta != (current.delta_x, current.delta_y)
                && delta != (-current.delta_x, -current.delta_y)
            {
                let next_x = current.x + delta.0;
                let next_y = current.y + delta.1;
                if coords.get(&(next_x, next_y)).is_some() {
                    heap.push(Reverse(Pos::new(
                        current.heat_loss + *coords.get(&(next_x, next_y)).unwrap(),
                        next_x,
                        next_y,
                        delta.0,
                        delta.1,
                        1,
                    )));
                }
            }
        }
    }
    panic!("should not happen");
}

fn two(input: &str) -> u32 {
    let mut coords: HashMap<(i32, i32), u32> = HashMap::default();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            coords.insert((x as i32, y as i32), c.to_digit(10).unwrap());
        }
    }
    let lines = input.split_terminator('\n').collect::<Vec<&str>>();
    let rows = lines.len() as i32;
    let cols = lines[0].chars().collect::<Vec<char>>().len() as i32;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(Pos::new(0, 0, 0, 0, 0, 0)));

    let mut seen: HashSet<Pos> = HashSet::default();
    while !heap.is_empty() {
        let current = heap.pop().unwrap().0;
        if current.x == rows - 1 && current.y == cols - 1 {
            return current.heat_loss;
        }

        if seen.contains(&current) {
            continue;
        }
        seen.insert(current);

        if current.step < 10 && (current.delta_x, current.delta_y) != (0, 0) {
            let next_x = current.x + current.delta_x;
            let next_y = current.y + current.delta_y;
            if coords.get(&(next_x, next_y)).is_some() {
                heap.push(Reverse(Pos::new(
                    current.heat_loss + *coords.get(&(next_x, next_y)).unwrap(),
                    next_x,
                    next_y,
                    current.delta_x,
                    current.delta_y,
                    current.step + 1,
                )));
            }
        }

        if current.step >= 4 || (current.delta_x, current.delta_y) == (0, 0) {
            for delta in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if delta != (current.delta_x, current.delta_y)
                    && delta != (-current.delta_x, -current.delta_y)
                {
                    let next_x = current.x + delta.0;
                    let next_y = current.y + delta.1;
                    if coords.get(&(next_x, next_y)).is_some() {
                        heap.push(Reverse(Pos::new(
                            current.heat_loss + *coords.get(&(next_x, next_y)).unwrap(),
                            next_x,
                            next_y,
                            delta.0,
                            delta.1,
                            1,
                        )));
                    }
                }
            }
        }
    }
    panic!("should not happend");
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        assert_eq!(102, one(input.trim()));
        assert_eq!(94, two(input.trim()));
    }

    #[test]
    fn test_answer() {
        println!("{}", one(&read_to_string("data/2023/day17.txt").unwrap()));
        println!("{}", two(&read_to_string("data/2023/day17.txt").unwrap()));
    }
}
