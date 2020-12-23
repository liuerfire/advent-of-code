use std::fs;

pub fn day10() {
    let content = fs::read_to_string("./src/year_2020/day10-input.txt").expect("shit");
    let mut lines: Vec<i32> = content.lines().map(|x| x.parse().unwrap()).collect();

    lines.sort();
    part1(&lines);

    lines.insert(0, 0);
    lines.push(lines[lines.len() - 1] + 3);
    part2(&lines);
}

fn part1(v: &Vec<i32>) {
    let mut diff1_num = 0;
    let mut diff3_num = 1;
    let mut current_rating = 0;

    for i in v {
        match i - current_rating {
            1 => {
                diff1_num += 1;
            }
            3 => {
                diff3_num += 1;
            }
            _ => {}
        }
        current_rating = i.clone();
    }
    println!("{}", diff1_num * diff3_num);
}

fn part2(v: &Vec<i32>) {
    let mut last = 0;
    let mut current: Vec<i32> = Vec::new();
    let mut result: u64 = 1;
    for i in v {
        if i - last == 3 || current.len() == 5 {
            if current.len() == 3 {
                result *= 2;
            } else if current.len() == 4 {
                result *= 4;
            } else if current.len() == 5 {
                result *= 7;
            }
            current.clear();
            current.push(*i);
        } else {
            current.push(*i);
        }
        last = *i;
    }
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10() {
        day10();
    }
}
