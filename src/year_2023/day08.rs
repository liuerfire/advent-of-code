use std::{collections::HashMap, str::Lines};

use regex::Regex;

struct Mapping {
    value: String,
    left: String,
    right: String,
}

impl Mapping {
    fn from_str(s: &str) -> Mapping {
        let re = Regex::new(r"(.{3}) = \((.{3}), (.{3})\)").unwrap();
        let caps = re.captures(s).unwrap();
        Mapping {
            value: caps[1].to_string(),
            left: caps[2].to_string(),
            right: caps[3].to_string(),
        }
    }

    fn next(&self, i: char) -> String {
        if i == 'L' {
            self.left.clone()
        } else {
            self.right.clone()
        }
    }
}

fn one(mut lines: Lines) -> u32 {
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();

    let mut maps: HashMap<String, Mapping> = HashMap::default();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let map = Mapping::from_str(line);
        maps.insert(map.value.clone(), map);
    }

    let mut s = 0;
    let mut i = 0;
    let mut r = "AAA".to_string();
    loop {
        s += 1;
        r = maps[&r].next(instructions[i % instructions.len()]);
        if r == "ZZZ" {
            break;
        }
        i += 1;
    }
    s
}

pub fn lcm(nums: Vec<u64>) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums[1..].to_vec());
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn two(mut lines: Lines) -> u64 {
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let mut maps: HashMap<String, Mapping> = HashMap::default();

    let mut p: Vec<String> = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let map = Mapping::from_str(line);
        if map.value.ends_with('A') {
            p.push(map.value.clone());
        }
        maps.insert(map.value.clone(), map);
    }

    let mut s = 0;
    let mut i = 0;

    let mut steps: HashMap<usize, u64> = HashMap::default();
    loop {
        let mut cycles = vec![];

        let mut j = 0;
        while j < p.len() {
            let n = maps[&p[j]].next(instructions[i % instructions.len()]);
            if n.ends_with('Z') {
                steps.insert(j, s + 1);
                if steps.len() == p.len() {
                    return lcm(steps.values().copied().collect());
                }
            }
            cycles.push(n);
            j += 1;
        }
        p = cycles;
        s += 1;
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!(2, one(input.trim().lines()));
        let input = r"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        assert_eq!(6, two(input.trim().lines()));
    }

    #[test]
    fn test_answer() {
        println!(
            "{}",
            one(read_to_string("data/2023/day08.txt")
                .unwrap()
                .trim()
                .lines())
        );
        println!(
            "{}",
            two(read_to_string("data/2023/day08.txt")
                .unwrap()
                .trim()
                .lines())
        );
    }
}
