use std::collections::HashMap;
use std::fs;

pub fn day9() {
    let content = fs::read_to_string("./src/year_2020/day9-input.txt").expect("shit");
    let lines: Vec<&str> = content.lines().collect();

    let mut invalid_number = 0;
    for i in 0..lines.len() - 25 {
        let target: i64 = lines[i + 25].parse().unwrap();
        if !has_target(&lines[i..i + 25], target) {
            println!("{}", target);
            invalid_number = target;
            break;
        }
    }

    for i in 0..lines.len() - 2 {
        let mut j = i + 2;
        let mut flag = false;
        loop {
            let n: Vec<i64> = lines[i..j]
                .iter()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            let sum: i64 = n.iter().sum();
            if sum == invalid_number {
                flag = true;
                break;
            }
            if sum > invalid_number {
                break;
            }
            j += 1;
        }
        if flag {
            let mut n: Vec<i64> = lines[i..j]
                .iter()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            n.sort();
            println!("{}", n[0] + n[n.len() - 1]);
            break;
        }
    }
}

fn has_target(numbers: &[&str], target: i64) -> bool {
    let mut map = HashMap::new();
    for (i, num) in numbers.iter().enumerate() {
        let n: i64 = num.parse().unwrap();
        match map.get(&(target - n)) {
            None => {
                map.insert(n, i);
            }
            _ => return true,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9() {
        day9();
    }
}
