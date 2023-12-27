use std::collections::HashMap;

// https://www.youtube.com/watch?v=bGWK76_e-LM
fn one(input: &str) -> i32 {
    let directions: HashMap<&str, (i32, i32)> =
        HashMap::from([("U", (0, -1)), ("D", (0, 1)), ("L", (-1, 0)), ("R", (1, 0))]);
    let mut paths: Vec<(i32, i32)> = vec![(0, 0)];
    let mut b = 0;
    for line in input.lines() {
        let splits = line.split_whitespace().collect::<Vec<&str>>();
        let dir = splits[0];
        let step: i32 = splits[1].parse().unwrap();
        b += step;
        let delta = *directions.get(&dir).unwrap();
        let current = *paths.last().unwrap();
        let (x, y) = (current.0 + step * delta.0, current.1 + step * delta.1);
        paths.push((x, y));
    }
    let mut aa: Vec<i32> = vec![];
    for i in 0..paths.len() {
        let mut ii = i;
        if ii == 0 {
            ii = paths.len() - 1;
        }
        aa.push(paths[i].0 * (paths[ii].1 - paths[(i + 1) % paths.len()].1));
    }
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let a = aa.iter().sum::<i32>().abs();
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    let i = a - b / 2 + 1;
    i + b
}

fn two(input: &str) -> i64 {
    let directions: HashMap<&str, (i64, i64)> =
        HashMap::from([("3", (0, -1)), ("1", (0, 1)), ("2", (-1, 0)), ("0", (1, 0))]);
    let mut paths: Vec<(i64, i64)> = vec![(0, 0)];
    let mut b = 0;
    for line in input.lines() {
        let hex = line.split_whitespace().collect::<Vec<&str>>()[2];

        let step = i64::from_str_radix(&hex[2..hex.len() - 2], 16).unwrap();
        let dir = &hex[hex.len() - 2..hex.len() - 1];

        b += step;
        let delta = *directions.get(&dir).unwrap();
        let current = *paths.last().unwrap();
        let (x, y) = (current.0 + step * delta.0, current.1 + step * delta.1);
        paths.push((x, y));
    }
    let mut aa: Vec<i64> = vec![];
    for i in 0..paths.len() {
        let mut ii = i;
        if ii == 0 {
            ii = paths.len() - 1;
        }
        aa.push(paths[i].0 * (paths[ii].1 - paths[(i + 1) % paths.len()].1));
    }
    let a = aa.iter().sum::<i64>().abs();
    let i = a - b / 2 + 1;
    i + b
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
        assert_eq!(62, one(input.trim()));
        assert_eq!(952408144115, two(input.trim()));
    }

    #[test]
    fn test_answer() {
        println!("{}", one(&read_to_string("data/2023/day18.txt").unwrap()));
        println!("{}", two(&read_to_string("data/2023/day18.txt").unwrap()));
    }
}
