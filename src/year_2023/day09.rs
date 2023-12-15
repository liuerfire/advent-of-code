use std::str::Lines;

fn one(lines: Lines) -> i64 {
    let mut s: Vec<i64> = vec![];
    for line in lines {
        let mut v: Vec<i64> = line
            .split_whitespace()
            .map(|i| i.parse::<i64>().unwrap())
            .collect();
        let mut lasts: Vec<i64> = vec![];
        loop {
            let mut t = vec![];
            let mut i = 0;
            while i < v.len() - 1 {
                t.push(v[i + 1] - v[i]);
                if i + 1 == v.len() - 1 {
                    lasts.push(v[i + 1]);
                }
                i += 1;
            }
            if t[0] == 0 && t[t.len() - 1] == 0 {
                break;
            }
            v = t;
        }
        s.push(lasts.iter().sum());
    }
    s.iter().sum()
}

fn two(lines: Lines) -> i64 {
    let mut s: Vec<i64> = vec![];
    for line in lines {
        let mut v: Vec<i64> = line
            .split_whitespace()
            .map(|i| i.parse::<i64>().unwrap())
            .collect();
        let mut firsts: Vec<i64> = vec![];
        loop {
            let mut t = vec![];
            let mut i = 0;
            while i < v.len() - 1 {
                t.push(v[i + 1] - v[i]);
                if i == 0 {
                    firsts.push(v[i]);
                }
                i += 1;
            }
            if t[0] == 0 && t[t.len() - 1] == 0 {
                break;
            }
            v = t;
        }
        let mut ss = 0;
        let mut i = firsts.len();
        while i > 0 {
            i -= 1;
            ss = firsts[i] - ss;
        }
        s.push(ss);
    }
    s.iter().sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
 0 3 6 9 12 15
 1 3 6 10 15 21
 10 13 16 21 30 45
 ";
        assert_eq!(114, one(input.trim().lines()));
        assert_eq!(2, two(input.trim().lines()));
    }

    #[test]
    fn test_answer() {
        println!(
            "{}",
            one(read_to_string("data/2023/day09.txt").unwrap().lines())
        );
        println!(
            "{}",
            two(read_to_string("data/2023/day09.txt").unwrap().lines())
        );
    }
}
