use std::{collections::HashMap, str::Lines};

fn one(lines: Lines) -> u32 {
    let mut s = 0;
    for line in lines {
        let (records, targets) = line.split_once(' ').unwrap();
        let targets = targets
            .split(',')
            .map(|i| i.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        s += count_arrangements(records.chars().collect(), &targets);
    }
    s
}

fn simplify(line: &[char]) -> String {
    let mut l: Vec<char> = vec![];
    let mut i = 0;
    while i < line.len() {
        l.push(line[i]);
        let mut j = i + 1;
        if line[i] == '.' {
            while j < line.len() && line[j] == '.' {
                j += 1;
                continue;
            }
        }
        i = j;
    }
    l.iter().collect()
}

fn is_valid(line: String, targets: &[u32]) -> bool {
    let splits = line.split('.').collect::<Vec<&str>>();
    let mut parts = vec![];
    for s in splits {
        if !s.is_empty() {
            parts.push(s);
        }
    }
    if parts.len() != targets.len() {
        return false;
    }
    for (i, t) in targets.iter().enumerate() {
        if parts[i].len() != *t as usize {
            return false;
        }
    }
    true
}

fn count_arrangements(line: Vec<char>, targets: &[u32]) -> u32 {
    let mut marks: Vec<usize> = vec![];
    for (i, c) in line.iter().enumerate() {
        if c == &'?' {
            marks.push(i);
        }
    }

    let mut s = 0;
    for i in 0..(1 << marks.len()) {
        let mut ll = line.clone();
        for j in 0..marks.len() {
            if i & (1 << j) == 0 {
                ll[marks[j]] = '#'
            } else {
                ll[marks[j]] = '.'
            }
        }
        if is_valid(simplify(&ll), targets) {
            s += 1;
        }
    }
    s
}

fn count<'a>(
    cache: &mut HashMap<(Vec<char>, &'a [u32]), u64>,
    line: Vec<char>,
    targets: &'a [u32],
) -> u64 {
    if line.is_empty() {
        if targets.is_empty() {
            return 1;
        }
        return 0;
    }
    if targets.is_empty() {
        if line.contains(&'#') {
            return 0;
        }
        return 1;
    }

    let key = (line.clone(), targets);

    if let Some(r) = cache.get(&key) {
        return *r;
    }

    let mut result = 0;

    if ['.', '?'].contains(&line[0]) {
        result += count(cache, line[1..].to_vec(), targets)
    }

    if ['#', '?'].contains(&line[0])
        && targets[0] <= line.len() as u32
        && !line[0..targets[0] as usize].contains(&'.')
        && (targets[0] as usize == line.len() || line[targets[0] as usize] != '#')
    {
        let mut l = vec![];
        let n = targets[0] as usize + 1;
        if n < line.len() {
            l = line[n..].to_vec();
        }
        result += count(cache, l.to_vec(), &targets[1..]);
    }

    cache.insert(key, result);

    result
}

fn two(lines: Lines) -> u64 {
    let mut s = 0;
    for line in lines {
        let (records, targets) = line.split_once(' ').unwrap();
        let targets = targets
            .split(',')
            .map(|i| i.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut a = vec![];
        let mut t = vec![];
        for _ in 0..5 {
            a.push(records);
            t.extend(targets.clone());
        }
        let l = a.join("?");
        let mut cache: HashMap<(Vec<char>, &[u32]), u64> = HashMap::default();
        s += count(&mut cache, l.chars().collect(), &t);
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
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        assert_eq!(21, one(input.trim().lines()));
        assert_eq!(525152, two(input.trim().lines()));
    }

    #[test]
    fn test_answer() {
        println!(
            "{}",
            one(read_to_string("data/2023/day12.txt").unwrap().lines())
        );
        println!(
            "{}",
            two(read_to_string("data/2023/day12.txt").unwrap().lines())
        );
    }
}
