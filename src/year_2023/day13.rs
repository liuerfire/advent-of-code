use std::cmp::min;

fn one(input: &str) -> usize {
    let mut s = 0;
    for part in input.split("\n\n") {
        s += find_reflection(columns(
            part.lines().map(String::from).collect::<Vec<String>>(),
        ));
        s += 100 * find_reflection(part.lines().map(String::from).collect::<Vec<String>>());
    }
    s
}

fn find_reflection(lines: Vec<String>) -> usize {
    let mut i = 1;
    while i < lines.len() {
        let mut left = lines[0..i].to_vec();
        left.reverse();
        let right = lines[i..].to_vec();
        let length = min(left.len(), right.len());
        if left[..length] == right[..length] {
            return i;
        }
        i += 1;
    }
    0
}

fn columns(lines: Vec<String>) -> Vec<String> {
    let mut data: Vec<Vec<char>> = vec![];
    for (row, l) in lines.iter().enumerate() {
        data.push(vec![]);
        for c in l.chars() {
            data[row].push(c);
        }
    }
    let mut r = vec![];
    for i in 0..data[0].len() {
        let mut s = String::default();
        for item in &data {
            s += &item[i].to_string();
        }
        r.push(s);
    }
    r
}

fn diff(left: &[String], right: &[String]) -> usize {
    let mut n = 0;
    for (left_s, right_s) in left.iter().zip(right.iter()) {
        for (left_c, right_c) in left_s
            .chars()
            .zip(right_s.chars().collect::<Vec<char>>().iter())
        {
            if left_c != *right_c {
                n += 1;
                if n > 1 {
                    return 2;
                }
            }
        }
    }
    n
}

fn find_reflection_2(lines: Vec<String>) -> usize {
    let mut i = 1;
    while i < lines.len() {
        let mut left = lines[0..i].to_vec();
        left.reverse();
        let right = lines[i..].to_vec();
        let length = min(left.len(), right.len());
        if diff(&left[..length], &right[..length]) == 1 {
            return i;
        }
        i += 1;
    }
    0
}

fn two(input: &str) -> usize {
    let mut s = 0;
    for part in input.split("\n\n") {
        s += find_reflection_2(columns(
            part.lines().map(String::from).collect::<Vec<String>>(),
        ));
        s += 100 * find_reflection_2(part.lines().map(String::from).collect::<Vec<String>>());
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
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        assert_eq!(405, one(input.trim()));
        assert_eq!(400, two(input.trim()));
    }

    #[test]
    fn test_answer() {
        println!("{}", one(&read_to_string("data/2023/day13.txt").unwrap()));
        println!("{}", two(&read_to_string("data/2023/day13.txt").unwrap()));
    }
}
