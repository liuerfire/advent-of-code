use std::{
    cmp::{max, min},
    str::Lines,
};

fn preprocess(lines: Lines) -> Vec<Vec<char>> {
    let mut data = vec![];
    for (i, line) in lines.enumerate() {
        data.push(vec![]);
        for c in line.chars() {
            data[i].push(c);
        }
    }
    data
}

fn one(lines: Vec<Vec<char>>) -> u32 {
    let mut s = 0;
    let rows = lines.len();
    let cols = lines[0].len();
    for i in 0..rows {
        let mut j = 0;
        while j < cols {
            if !lines[i][j].is_ascii_digit() {
                j += 1;
                continue;
            }
            let mut k = j;
            let mut num = 0;
            while k < cols && lines[i][k].is_ascii_digit() {
                num = 10 * num + lines[i][k].to_digit(10).unwrap();
                k += 1;
            }
            let mut adjacent = false;
            for x in max(0, j.saturating_sub(1))..min(cols, k + 1) {
                let ii = i.saturating_sub(1);
                if (ii > 0) && !lines[ii][x].is_ascii_digit() && lines[ii][x] != '.' {
                    adjacent = true
                }
                if !lines[i][x].is_ascii_digit() && lines[i][x] != '.' {
                    adjacent = true
                }
                if (i + 1 < rows) && !lines[i + 1][x].is_ascii_digit() && lines[i + 1][x] != '.' {
                    adjacent = true
                }
            }
            if adjacent {
                s += num;
            }
            j = k;
        }
    }
    s
}

fn two(lines: Vec<Vec<char>>) -> u32 {
    let mut s = 0;
    let rows = lines.len();
    let cols = lines[0].len();

    let mut adjacents: Vec<Vec<u32>> = vec![];
    for _ in 0..rows * cols {
        adjacents.push(vec![]);
    }
    for i in 0..rows {
        let mut j = 0;
        while j < cols {
            if !lines[i][j].is_ascii_digit() {
                j += 1;
                continue;
            }
            let mut k = j;
            let mut num = 0;
            while k < cols && lines[i][k].is_ascii_digit() {
                num = 10 * num + lines[i][k].to_digit(10).unwrap();
                k += 1;
            }
            for x in max(0, j.saturating_sub(1))..min(cols, k + 1) {
                if let Some(ii) = i.checked_sub(1) {
                    if lines[ii][x] == '*' {
                        adjacents[ii * cols + x].push(num);
                    }
                }
                if lines[i][x] == '*' {
                    adjacents[i * cols + x].push(num);
                }
                if i + 1 < rows && lines[i + 1][x] == '*' {
                    adjacents[(i + 1) * cols + x].push(num);
                }
            }
            j = k;
        }
    }
    for i in adjacents {
        if i.len() == 2 {
            s += i[0] * i[1]
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        assert_eq!(4361, one(preprocess(input.trim().lines())));
        assert_eq!(467835, two(preprocess(input.trim().lines())));
    }

    #[test]
    fn test_solve() {
        println!(
            "{}",
            one(preprocess(
                read_to_string("data/2023/day03.txt").unwrap().lines()
            ))
        );
        println!(
            "{}",
            two(preprocess(
                read_to_string("data/2023/day03.txt").unwrap().lines()
            ))
        );
    }
}
