use std::collections::HashMap;
use std::fs;

pub fn day6() {
    let content = fs::read_to_string("./src/year_2020/day6-input.txt").expect("shit");
    let mut sum1 = 0;
    let mut sum2 = 0;
    for entry in content.split("\n\n").collect::<Vec<&str>>() {
        sum1 += part1(entry);
        sum2 += part2(entry);
    }
    println!("part1: {}", sum1);
    println!("part2: {}", sum2);
}

fn part1(s: &str) -> usize {
    let mut ret: HashMap<String, i32> = HashMap::new();
    for line in s.split_whitespace().collect::<Vec<&str>>() {
        for c in line.chars() {
            ret.insert(c.to_string(), 1);
        }
    }
    ret.len()
}

fn part2(s: &str) -> usize {
    let sets: Vec<String> = s.split_whitespace().map(String::from).collect();
    let mut q: Vec<String> = sets[0].chars().map(String::from).collect();
    for i in 1..sets.len() {
        let s: Vec<String> = sets[i].chars().map(String::from).collect();
        q = intersection(&q, &s);
    }
    q.len()
}

fn intersection(a: &[String], b: &[String]) -> Vec<String> {
    let mut h1 = HashMap::new();
    for i in a {
        h1.insert(i, 1);
    }
    let mut h2 = HashMap::new();
    for i in b {
        h2.insert(i, 1);
    }
    let mut v = Vec::new();
    for i in a {
        if h2.contains_key(i) {
            v.push(i.to_string());
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6() {
        day6();
    }
}
