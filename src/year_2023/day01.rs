use std::{char, str::Lines};

fn solve1(lines: Lines) -> u32 {
    let mut sum = 0;
    for line in lines {
        let l: Vec<char> = line.chars().collect();

        let mut i = 0;
        let mut left = 0;
        let mut right = 0;
        while i < l.len() {
            if let Some(num) = l[i].to_digit(10) {
                if left == 0 {
                    left = num;
                }
            }
            if let Some(num) = l[l.len() - 1 - i].to_digit(10) {
                if right == 0 {
                    right = num;
                }
            }
            i += 1;
        }
        sum += left * 10 + right;
    }
    sum
}

fn solve2(lines: Lines) -> u32 {
    let mut sum = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let words = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let mut s = String::default();
        for i in 0..line.len() {
            for (idx, word) in words.iter().enumerate() {
                if word.len() <= line[i..].len() && &line[i..i + word.len()] == *word {
                    let c = char::from_digit(idx as u32 + 1, 10).unwrap();
                    s.push(c);
                    break;
                }
            }
            if let Some(n) = line.chars().nth(i).unwrap().to_digit(10) {
                s.push(char::from_digit(n, 10).unwrap());
            }
        }

        let first: u32 = s[0..1].parse().unwrap();
        let last: u32 = s[s.len() - 1..].parse().unwrap();
        sum += first * 10 + last;
    }
    sum
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        assert_eq!(142, solve1(input.lines()));
        let input = r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        solve2(input.lines());
        assert_eq!(281, solve2(input.lines()));
    }

    #[test]
    fn test_solve() {
        println!(
            "{}",
            solve1(read_to_string("data/2023/day01.txt").unwrap().lines())
        );
        println!(
            "{}",
            solve2(read_to_string("data/2023/day01.txt").unwrap().lines())
        );
    }
}
