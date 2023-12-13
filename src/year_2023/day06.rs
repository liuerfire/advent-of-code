fn one(input: &str) -> u32 {
    let splits: Vec<&str> = input.split_terminator('\n').collect();
    let times: Vec<u32> = splits[0]
        .split_whitespace()
        .filter_map(|i| i.parse::<u32>().ok())
        .collect();
    let distances: Vec<u32> = splits[1]
        .split_whitespace()
        .filter_map(|i| i.parse::<u32>().ok())
        .collect();
    let mut s = 1;
    for i in 0..times.len() {
        let mut over = 0;
        for x in 0..times[i] + 1 {
            let d = x * (times[i] - x);
            if d > distances[i] {
                over += 1;
            }
        }
        s *= over;
    }
    s
}

fn two(input: &str) -> u64 {
    let splits: Vec<&str> = input.split_terminator('\n').collect();
    let mut ts = String::default();
    let mut ds = String::default();
    for (idx, i) in splits[0].split_whitespace().enumerate() {
        if idx == 0 {
            continue;
        }
        ts += i;
    }
    for (idx, i) in splits[1].split_whitespace().enumerate() {
        if idx == 0 {
            continue;
        }
        ds += i;
    }
    let times: u64 = ts.parse().unwrap();
    let distances: u64 = ds.parse().unwrap();
    let mut s: u64 = 1;
    let mut over = 0;
    for x in 0..times + 1 {
        let d = x * (times - x);
        if d > distances {
            over += 1;
        }
    }
    s *= over;
    s
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!(288, one(input.trim()));
    }

    #[test]
    fn test_answer() {
        println!("{}", one(&read_to_string("data/2023/day06.txt").unwrap()));
        println!("{}", two(&read_to_string("data/2023/day06.txt").unwrap()));
    }
}
