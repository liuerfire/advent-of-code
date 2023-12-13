use std::cmp::{max, min};

fn parse_part(s: &str, seed: i64) -> i64 {
    let lines: Vec<&str> = s.split_terminator('\n').collect();
    for l in &lines[1..] {
        let [dst, src, range] = l
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect::<Vec<i64>>()
            .try_into()
            .unwrap();
        if seed >= src && seed < src + range {
            return seed + dst - src;
        }
    }
    seed
}

fn one(parts: Vec<&str>) -> i64 {
    let seeds: Vec<i64> = parts[0]
        .split_whitespace()
        .filter_map(|i| i.parse::<i64>().ok())
        .collect();
    let mut locations: Vec<i64> = vec![];
    for seed in seeds {
        let mut location = seed;
        for part in &parts[1..] {
            location = parse_part(part, location);
        }
        locations.push(location);
    }
    *locations.iter().min().unwrap()
}

fn parse_range(part: &str, seeds: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut locations = vec![];
    let mut tmp = seeds;
    let mappings: Vec<&str> = part.split_terminator('\n').collect();

    for (idx, mapping) in mappings[1..].iter().enumerate() {
        let [dst, src, range] = mapping
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect::<Vec<i64>>()
            .try_into()
            .unwrap();
        let src_range = (src, src + range);
        let offset = dst - src;

        let mut i = 0;
        let mut next = vec![];
        loop {
            if idx >= mappings[1..].len() {
                break;
            }
            if i >= tmp.len() {
                break;
            }
            if tmp[i].1 < src_range.0 || tmp[i].0 >= src_range.1 {
                next.push((tmp[i].0, tmp[i].1));
                i += 1;
                continue;
            }
            let inner = (max(tmp[i].0, src_range.0), min(tmp[i].1, src_range.1));
            let left = (tmp[i].0, inner.0);
            let right = (inner.1, tmp[i].1);
            if left.0 < left.1 {
                next.push(left);
            }
            if right.0 < right.1 {
                next.push(right);
            }
            locations.push((inner.0 + offset, inner.1 + offset));
            i += 1;
        }
        tmp = next;
    }
    locations.extend(tmp);
    locations
}

fn two(parts: Vec<&str>) -> i64 {
    let seeds: Vec<i64> = parts[0]
        .split_whitespace()
        .filter_map(|i| i.parse::<i64>().ok())
        .collect();
    let mut i = 0;
    let mut locations = vec![];
    while i < seeds.len() {
        let mut l = vec![(seeds[i], seeds[i] + seeds[i + 1])];
        for part in &parts[1..] {
            l = parse_range(part, l);
        }
        locations.extend(l);
        i += 2;
    }
    locations.iter().map(|i| i.0).min().unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!(35, one(input.trim().split_terminator("\n\n").collect()));
        assert_eq!(46, two(input.trim().split_terminator("\n\n").collect()));
    }

    #[test]
    fn test_solve() {
        println!(
            "{}",
            one(read_to_string("data/2023/day05.txt")
                .unwrap()
                .split_terminator("\n\n")
                .collect())
        );
        println!(
            "{}",
            two(read_to_string("data/2023/day05.txt")
                .unwrap()
                .split_terminator("\n\n")
                .collect())
        );
    }
}
