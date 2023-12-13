use std::{cmp::Ordering, collections::HashMap, fmt};

struct Card {
    labels: Vec<char>,
    point: u32,
    strength: u32,
}

fn parse_strength_v1(labels: &Vec<char>) -> u32 {
    let mut ret: HashMap<char, u32> = HashMap::default();
    for c in labels {
        *ret.entry(*c).or_default() += 1
    }
    let mut v: Vec<u32> = ret.values().cloned().collect();
    v.sort();
    if v.eq(&[5]) {
        return 6;
    }
    if v.eq(&[1, 4]) {
        return 5;
    }
    if v.eq(&[2, 3]) {
        return 4;
    }
    if v.eq(&[1, 1, 3]) {
        return 3;
    }
    if v.eq(&[1, 2, 2]) {
        return 2;
    }
    if v.eq(&[1, 1, 1, 2]) {
        return 1;
    }
    0
}

fn parse_label_v1(label: char) -> u32 {
    match label {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => label.to_digit(10).unwrap(),
    }
}

fn parse_strength_v2(labels: &Vec<char>) -> u32 {
    let mut ret: HashMap<char, u32> = HashMap::default();
    for c in labels {
        *ret.entry(*c).or_default() += 1
    }
    let mut v: Vec<u32> = ret.values().cloned().collect();
    v.sort();
    if v.eq(&[5]) {
        return 6;
    }
    let j = *ret.entry('J').or_default();
    if v.eq(&[1, 4]) {
        if j == 1 {
            return 6;
        }
        if j == 4 {
            return 6;
        }
        return 5;
    }
    if v.eq(&[2, 3]) {
        if j == 2 || j == 3 {
            return 6;
        }
        return 4;
    }
    if v.eq(&[1, 1, 3]) {
        if j == 1 || j == 3 {
            return 5;
        }
        return 3;
    }
    if v.eq(&[1, 2, 2]) {
        if j == 1 {
            return 4;
        }
        if j == 2 {
            return 5;
        }
        return 2;
    }
    if v.eq(&[1, 1, 1, 2]) {
        if j == 1 || j == 2 {
            return 3;
        }
        return 1;
    }
    if j == 1 {
        return 1;
    }
    0
}

fn parse_label_v2(label: char) -> u32 {
    match label {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        'J' => 1,
        _ => label.to_digit(10).unwrap(),
    }
}

impl Card {
    fn from_str(s: &str, v2: bool) -> Self {
        let splits = s.split_whitespace().collect::<Vec<&str>>();
        let labels = splits[0].chars().collect();
        let point: u32 = splits[1].parse().unwrap();

        let mut strength = parse_strength_v1(&labels);
        if v2 {
            strength = parse_strength_v2(&labels);
        }
        Self {
            point,
            labels,
            strength,
        }
    }

    fn cmp_v1(&self, other: &Card) -> Option<Ordering> {
        if self.strength > other.strength {
            return Some(Ordering::Greater);
        }
        if self.strength < other.strength {
            return Some(Ordering::Less);
        }
        for i in 0..self.labels.len() {
            if parse_label_v1(self.labels[i]) > parse_label_v1(other.labels[i]) {
                return Some(Ordering::Greater);
            }
            if parse_label_v1(self.labels[i]) < parse_label_v1(other.labels[i]) {
                return Some(Ordering::Less);
            }
        }
        Some(Ordering::Equal)
    }

    fn cmp_v2(&self, other: &Card) -> Option<Ordering> {
        if self.strength > other.strength {
            return Some(Ordering::Greater);
        }
        if self.strength < other.strength {
            return Some(Ordering::Less);
        }
        for i in 0..self.labels.len() {
            if parse_label_v2(self.labels[i]) > parse_label_v2(other.labels[i]) {
                return Some(Ordering::Greater);
            }
            if parse_label_v2(self.labels[i]) < parse_label_v2(other.labels[i]) {
                return Some(Ordering::Less);
            }
        }
        Some(Ordering::Equal)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .labels
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{} {}", s, self.point)
    }
}

fn one(input: &str) -> u32 {
    let mut cards: Vec<Card> = input
        .trim()
        .lines()
        .map(|i| Card::from_str(i, false))
        .collect();
    cards.sort_by(|a, b| a.cmp_v1(b).unwrap());
    let mut s = 0;
    for (i, card) in cards.iter().enumerate() {
        s += (i as u32 + 1) * card.point;
    }
    s
}

fn two(input: &str) -> u32 {
    let mut cards: Vec<Card> = input
        .trim()
        .lines()
        .map(|i| Card::from_str(i, true))
        .collect();
    cards.sort_by(|a, b| a.cmp_v2(b).unwrap());
    let mut s = 0;
    for (i, card) in cards.iter().enumerate() {
        // println!("{}", card);
        s += (i as u32 + 1) * card.point;
    }
    s
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        assert_eq!(6440, one(input));
    }

    #[test]
    fn test_answer() {
        println!("{}", one(&read_to_string("data/2023/day07.txt").unwrap()));
        println!("{}", two(&read_to_string("data/2023/day07.txt").unwrap()));
    }
}
