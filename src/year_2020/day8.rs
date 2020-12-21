use std::fs;

fn parse(line: &str) -> (String, i32) {
    let t: Vec<&str> = line.split(" ").collect();
    (t[0].to_string(), t[1].parse().unwrap())
}

pub fn day8() {
    let content = fs::read_to_string("./src/year_2020/day8-input.txt").expect("shit");
    let lines = content.lines().collect();
    println!("{}", part1(&lines));
    part2(&lines);
}

fn part1(lines: &Vec<&str>) -> i32 {
    let mut accumulator = 0;
    let mut i: usize = 0;
    let mut visited = Vec::new();
    while !visited.contains(&i) {
        visited.push(i);
        let t: Vec<&str> = lines[i].split(" ").collect();
        let op = t[0];
        let num: i32 = t[1].parse().unwrap();

        match op {
            "acc" => {
                accumulator += num;
                i += 1;
            }
            "jmp" => {
                i = (i as i32 + num) as usize;
            }
            "nop" => {
                i += 1;
            }
            _ => {}
        }
    }
    accumulator
}

fn run(lines: Vec<&str>, flip: usize) -> (i32, bool) {
    let mut accumulator = 0;
    let mut i: usize = 0;
    let mut visited = Vec::new();
    loop {
        if visited.contains(&i) {
            return (0, false);
        }
        visited.push(i);
        let (op, num) = parse(lines[i]);
        let mut new_op = op.as_str();
        if i == flip {
            new_op = if op == "jmp" {
                "nop"
            } else if op == "nop" {
                "jmp"
            } else {
                "acc"
            };
        }
        match new_op {
            "acc" => {
                accumulator += num;
                i += 1;
            }
            "jmp" => {
                i = (i as i32 + num) as usize;
            }
            "nop" => {
                i += 1;
            }
            _ => {}
        }
        if i == lines.len() - 1 {
            break;
        }
    }
    (accumulator, true)
}

fn part2(lines: &Vec<&str>) {
    for i in 0..lines.len() {
        let (a, b) = run(lines.clone(), i);
        if b {
            println!("{}", a);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8() {
        day8();
    }
}
