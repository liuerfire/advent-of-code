use std::collections::HashMap;
use std::fs;

fn read_input() -> Vec<String> {
    let content =
        fs::read_to_string("./src/year_2020/day1-input.txt").expect("can't read the file");
    content.trim().split("\n").map(String::from).collect()
}

fn two_sum(numbers: Vec<String>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(numbers.len());
    for (i, num) in numbers.iter().enumerate() {
        let n: i32 = num.parse().unwrap();
        match map.get(&(target - n)) {
            None => {
                map.insert(n, i);
            }
            _ => return vec![n, target - n],
        }
    }
    vec![]
}

pub fn multiply_two() {
    let numbers = read_input();

    let r = two_sum(numbers, 2020);

    println!("{}", r[0] * r[1]);
}

pub fn multiply_three() {
    let numbers = read_input();

    for num in numbers.iter() {
        let n: i32 = num.parse().unwrap();
        let t = two_sum(numbers.clone(), 2020 - n);
        if t.len() == 2 {
            println!("{}", t[0] * t[1] * n);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        multiply_two();
        multiply_three();
    }
}
