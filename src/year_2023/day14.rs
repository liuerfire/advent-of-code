use std::collections::HashMap;

fn one(lines: &[String]) -> usize {
    let mut data = vec![];
    for (i, line) in lines.iter().enumerate() {
        data.push(vec![]);
        for c in line.chars() {
            data[i].push(c);
        }
    }
    let mut i = 0;
    while i < data[0].len() {
        let mut k = 0;
        while k < data.len() {
            let mut j = 1;
            while j < data.len() {
                if data[j][i] == 'O' && data[j - 1][i] == '.' {
                    data[j][i] = '.';
                    data[j - 1][i] = 'O';
                }
                j += 1;
            }
            k += 1;
        }
        i += 1;
    }
    let mut s = 0;
    for (i, d) in data.iter().enumerate() {
        let mut ss = 0;
        for j in d {
            if j == &'O' {
                ss += 1;
            }
        }
        s += ss * (data.len() - i);
    }
    s
}

fn rotate(mut data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let row = data.len();
    let col = data[0].len();

    let mut i = 0;
    while i < data[0].len() {
        let mut k = 0;
        while k < data.len() {
            let mut j = 1;
            while j < data.len() {
                if data[j][i] == 'O' && data[j - 1][i] == '.' {
                    data[j][i] = '.';
                    data[j - 1][i] = 'O';
                }
                j += 1;
            }
            k += 1;
        }
        i += 1;
    }

    let mut ret: Vec<Vec<char>> = vec![];
    for i in 0..col {
        ret.push(vec![]);
        for _ in 0..row {
            ret[i].push(' ');
        }
    }

    let mut r = 0;
    while r < data.len() {
        let mut c = 0;
        while c < data[0].len() {
            ret[c][data.len() - 1 - r] = data[r][c];
            c += 1;
        }
        r += 1;
    }

    ret
}

fn cycle(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // north
    let mut ret = rotate(data);
    // west
    ret = rotate(ret);
    // south
    ret = rotate(ret);
    // east
    rotate(ret)
}

fn two(mut data: Vec<Vec<char>>) -> usize {
    let mut seen: HashMap<Vec<Vec<char>>, u32> = HashMap::default();
    seen.insert(data.clone(), 0);

    let n = 1000000000;

    let mut i = 0;
    loop {
        i += 1;
        data = cycle(data.clone());
        if seen.get(&data).is_some() {
            break;
        }
        seen.insert(data.clone(), i);
    }
    let x = *seen.get(&data).unwrap();
    for _ in 0..((n - x) % (i - x)) {
        data = cycle(data);
    }
    let mut s = 0;
    for (i, d) in data.iter().enumerate() {
        let mut ss = 0;
        for j in d {
            if j == &'O' {
                ss += 1;
            }
        }
        s += ss * (data.len() - i);
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
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
        assert_eq!(
            136,
            one(&input
                .trim()
                .split('\n')
                .map(String::from)
                .collect::<Vec<String>>())
        );
        assert_eq!(
            64,
            two(input
                .trim()
                .split('\n')
                .map(|i| i.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>())
        );
    }

    #[test]
    fn test_answer() {
        println!(
            "{}",
            one(&read_to_string("data/2023/day14.txt")
                .unwrap()
                .trim()
                .split('\n')
                .map(String::from)
                .collect::<Vec<String>>())
        );
        println!(
            "{}",
            two(read_to_string("data/2023/day14.txt")
                .unwrap()
                .trim()
                .split('\n')
                .map(|i| i.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>())
        );
    }
}
