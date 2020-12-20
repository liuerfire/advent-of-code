use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Color(String, i32);

pub fn day7() {
    let re1 = Regex::new(r"^([^ ]+ [^ ]+) bags contain (.*)$").unwrap();
    let re2 = Regex::new(r"(\d+) ([^ ]+ [^ ]+)").unwrap();

    let mut colors: HashMap<String, Vec<Color>> = HashMap::new();
    let mut parents: HashMap<String, Vec<String>> = HashMap::new();

    let content = fs::read_to_string("./src/year_2020/day7-input.txt").expect("shit");
    for line in content.lines() {
        let mat = re1.captures(line).unwrap();

        let k = mat[1].to_string();

        let mut color: Vec<Color> = Vec::new();
        for cap in re2.captures_iter(&mat[2]) {
            let c = cap[2].to_string();
            color.push(Color(c.clone(), cap[1].parse().unwrap()));

            let v = parents.entry(c.clone()).or_insert(vec![]);
            v.push(k.clone());
        }
        colors.insert(k, color);
    }

    println!("{}", part1(&parents));
    println!("{}", part2(&colors));
}

fn part1(colors: &HashMap<String, Vec<String>>) -> usize {
    let mut total = Vec::new();
    let mut p = colors.get("shiny gold").unwrap().clone();
    while p.len() > 0 {
        let color = p.pop().unwrap();
        if !total.contains(&color) {
            total.push(color.clone());
            let v = colors.get(&color).unwrap_or(&vec![]).clone();
            p.extend(v);
        }
    }
    total.len()
}

fn part2(colors: &HashMap<String, Vec<Color>>) -> i32 {
    let mut total = 0;
    let mut children = colors.get("shiny gold").unwrap().clone();
    while children.len() > 0 {
        let Color(color, num) = children.pop().unwrap();
        total += num;
        for Color(c, n) in colors.get(&color).unwrap_or(&vec![]) {
            children.push(Color(c.clone(), n * num));
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7() {
        day7();
    }
}
