use std::fs;

pub fn trees() {
    let content = fs::read_to_string("./src/year_2020/day3-input.txt").expect("shit");
    let lines: Vec<&str> = content.trim().split("\n").collect();

    let a = count(lines.clone(), 1, 1);
    let b = count(lines.clone(), 3, 1);
    let c = count(lines.clone(), 5, 1);
    let d = count(lines.clone(), 7, 1);
    let e = count(lines.clone(), 1, 2);

    println!(
        "{}",
        a.checked_mul(b)
            .unwrap()
            .checked_mul(c)
            .unwrap()
            .checked_mul(d)
            .unwrap()
            .checked_mul(e)
            .unwrap(),
    );
}

fn count(lines: Vec<&str>, right: usize, down: usize) -> usize {
    let mut count = 0;
    for (i, line) in lines.iter().enumerate().step_by(down) {
        let idx = right * (i / down) % line.len();
        // let idx = right * i % line.len();
        if line.chars().nth(idx).unwrap().to_string() == "#" {
            count += 1
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trees() {
        trees();
    }
}
