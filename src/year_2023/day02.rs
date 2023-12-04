use std::{collections::HashMap, str::Lines};

fn one(lines: Lines) -> u32 {
    let mut s = 0;
    for line in lines {
        let splits: Vec<&str> = line.split(": ").collect();
        let id_splits = splits[0].split_whitespace().collect::<Vec<&str>>();
        let id = id_splits.last().unwrap();
        let mut possible = true;
        for part in splits[1].split("; ") {
            let mut counter: HashMap<&str, u32> = HashMap::default();
            for set in part.split(", ") {
                let (number, color) = set.split_once(" ").unwrap();
                let n = number.parse::<u32>().unwrap();
                *counter.entry(color).or_insert(0) += n;
            }
            for (k, v) in counter {
                if (k == "red" && v > 12) || (k == "green" && v > 13) || (k == "blue" && v > 14) {
                    possible = false;
                    break;
                }
            }
        }
        if possible {
            s += id.parse::<u32>().unwrap();
        }
    }
    s
}

fn two(lines: Lines) -> u32 {
    let mut s = 0;
    for line in lines {
        let splits: Vec<&str> = line.split(": ").collect();
        let mut counter: HashMap<&str, u32> = HashMap::default();
        counter.insert("red", 0);
        counter.insert("green", 0);
        counter.insert("blue", 0);
        for part in splits[1].split("; ") {
            for set in part.split(", ") {
                let (number, color) = set.split_once(" ").unwrap();
                let n = number.parse::<u32>().unwrap();
                if n > *counter.get(color).unwrap() {
                    *counter.get_mut(color).unwrap() = n;
                }
            }
        }
        s += counter["red"] * counter["green"] * counter["blue"]
    }
    s
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, one(input.lines()));
        assert_eq!(2286, two(input.lines()));
    }

    #[test]
    fn test_solve() {
        println!(
            "{}",
            one(read_to_string("data/2023/day02.txt").unwrap().lines())
        );
        println!(
            "{}",
            two(read_to_string("data/2023/day02.txt").unwrap().lines())
        );
    }
}
