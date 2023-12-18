use std::{
    cmp::{max, min},
    str::Lines,
};

fn one(lines: Lines) -> u32 {
    let mut data = vec![];
    let mut galaxies = vec![];
    for (r, line) in lines.enumerate() {
        data.push(vec![]);
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((r, col));
            }
            data[r].push(c);
        }
    }

    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];

    for (row, line) in data.iter().enumerate() {
        if !line.contains(&'#') {
            empty_rows.push(row);
        }
    }

    for col in 0..data[0].len() {
        let mut galaxy = false;
        for line in &data {
            if line[col] == '#' {
                galaxy = true;
                break;
            }
        }
        if !galaxy {
            empty_cols.push(col);
        }
    }

    let mut s = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            s += distance(galaxies[i], galaxies[j], &empty_rows, &empty_cols);
        }
    }
    s
}

fn distance(
    a: (usize, usize),
    b: (usize, usize),
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
) -> u32 {
    let (r1, r2) = (min(a.0, b.0), max(a.0, b.0));
    let (c1, c2) = (min(a.1, b.1), max(a.1, b.1));

    let mut d = 0;
    for r in r1..r2 {
        d += 1;
        if empty_rows.contains(&r) {
            d += 1;
        }
    }
    for c in c1..c2 {
        d += 1;
        if empty_cols.contains(&c) {
            d += 1;
        }
    }
    d
}

fn two(lines: Lines) -> u64 {
    let mut data = vec![];
    let mut galaxies = vec![];
    for (r, line) in lines.enumerate() {
        data.push(vec![]);
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((r, col));
            }
            data[r].push(c);
        }
    }

    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];

    for (row, line) in data.iter().enumerate() {
        if !line.contains(&'#') {
            empty_rows.push(row);
        }
    }

    for col in 0..data[0].len() {
        let mut galaxy = false;
        for line in &data {
            if line[col] == '#' {
                galaxy = true;
                break;
            }
        }
        if !galaxy {
            empty_cols.push(col);
        }
    }

    let mut s = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            s += distance2(galaxies[i], galaxies[j], &empty_rows, &empty_cols);
        }
    }
    s
}

fn distance2(
    a: (usize, usize),
    b: (usize, usize),
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
) -> u64 {
    let (r1, r2) = (min(a.0, b.0), max(a.0, b.0));
    let (c1, c2) = (min(a.1, b.1), max(a.1, b.1));

    let mut d = 0;
    for r in r1..r2 {
        d += 1;
        if empty_rows.contains(&r) {
            d += 1000000 - 1;
        }
    }
    for c in c1..c2 {
        d += 1;
        if empty_cols.contains(&c) {
            d += 1000000 - 1;
        }
    }
    d
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        assert_eq!(374, one(input.trim().lines()));
    }

    #[test]
    fn test_answer() {
        println!(
            "{}",
            one(read_to_string("data/2023/day11.txt").unwrap().lines())
        );
        println!(
            "{}",
            two(read_to_string("data/2023/day11.txt").unwrap().lines())
        );
    }
}
