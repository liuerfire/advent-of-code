use std::{
    collections::{HashMap, HashSet},
    str::Lines,
};

fn one(lines: Lines) -> u32 {
    let mut s = 0;
    for line in lines {
        let (_, numbers) = line.split_once(':').unwrap();
        let (winnings, owns) = numbers.split_once('|').unwrap();
        let wins = winnings
            .split_whitespace()
            .map(|i| i.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let owns = owns
            .split_whitespace()
            .map(|i| i.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let c = owns.intersection(&wins).count();
        if c > 0 {
            s += 2u32.pow(c as u32 - 1);
        }
    }
    s
}

fn two(lines: Lines) -> u32 {
    let mut copies: HashMap<usize, u32> = HashMap::default();

    for (i, line) in lines.enumerate() {
        *copies.entry(i).or_default() += 1;
        let (_, numbers) = line.split_once(':').unwrap();
        let (winnings, owns) = numbers.split_once('|').unwrap();
        let wins = winnings
            .split_whitespace()
            .map(|i| i.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let owns = owns
            .split_whitespace()
            .map(|i| i.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let c = owns.intersection(&wins).count();
        for j in 0..c {
            *copies.entry(i + 1 + j).or_default() += copies[&i];
        }
    }

    copies.values().sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        assert_eq!(13, one(input.trim().lines()));
        assert_eq!(30, two(input.trim().lines()));
    }

    #[test]
    fn test_solve() {
        println!(
            "{}",
            one(read_to_string("data/2023/day04.txt").unwrap().lines())
        );
        println!(
            "{}",
            two(read_to_string("data/2023/day04.txt").unwrap().lines())
        );
    }
}
