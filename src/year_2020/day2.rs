use std::collections::HashMap;
use std::fs;

pub fn valid_passwords() {
    let content = fs::read_to_string("./src/year_2020/day2-input.txt").expect("shit");
    let lines: Vec<&str> = content.trim().split("\n").collect();
    let mut n = 0;
    for l in lines.iter() {
        if check1(l) {
            n += 1;
        }
    }
    println!("{}", n);

    n = 0;
    for l in lines.iter() {
        if check2(l) {
            n += 1;
        }
    }
    println!("{}", n);
}

fn check1(s: &str) -> bool {
    let ss: Vec<&str> = s.split(" ").collect();

    let x: Vec<&str> = ss[0].split("-").collect();
    let min = x[0];
    let max = x[1];

    let key: Vec<&str> = ss[1].split(":").collect();
    let key = key[0];

    let mut map: HashMap<String, i32> = HashMap::new();
    for i in ss[2].chars() {
        let t = i.to_string();
        match map.get(&t) {
            None => {
                map.insert(t, 1);
            }
            _ => {
                *map.get_mut(&t).unwrap() += 1;
            }
        }
    }
    if map.contains_key(key) {
        if map[key] >= min.parse().unwrap() && map[key] <= max.parse().unwrap() {
            return true;
        }
    }
    false
}

fn check2(s: &str) -> bool {
    let ss: Vec<&str> = s.split(" ").collect();

    let x: Vec<&str> = ss[0].split("-").collect();
    let pos1 = x[0].parse().unwrap();
    let pos2 = x[1].parse().unwrap();

    let key: Vec<&str> = ss[1].split(":").collect();
    let key = key[0];

    let mut flag = false;
    for (i, c) in ss[2].chars().enumerate() {
        let idx = i + 1;
        let k = c.to_string();
        if idx == pos1 && k == key {
            flag = true;
        }
        if idx == pos2 {
            if flag {
                if k != key {
                    return true;
                }
            } else {
                if k == key {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_passwords() {
        valid_passwords();
    }
}
