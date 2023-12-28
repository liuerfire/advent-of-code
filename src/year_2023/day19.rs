use std::{
    cmp::{max, min},
    collections::HashMap,
};

#[derive(Debug, Clone)]
enum Rule<'a> {
    One(&'a str),
    Cmp {
        var: &'a str,
        op: &'a str,
        value: u32,
        target: &'a str,
    },
}

fn eval_rules(
    workflows: &HashMap<&str, Vec<Rule>>,
    rating: &HashMap<&str, u32>,
    name: &str,
) -> u32 {
    let s = rating.values().sum();
    if name == "A" {
        return s;
    }
    if name == "R" {
        return 0;
    }
    let rules = workflows.get(name).unwrap();
    for rule in rules {
        for (var, value) in rating {
            if let Some(target) = match_rule(rule, var, *value) {
                return eval_rules(workflows, rating, target);
            }
        }
    }
    if let Rule::One(ch) = rules.last().unwrap() {
        return eval_rules(workflows, rating, ch);
    }
    unreachable!();
}

fn match_rule<'a>(rule: &'a Rule, v: &str, va: u32) -> Option<&'a str> {
    if let Rule::Cmp {
        var,
        op,
        value,
        target,
    } = rule
    {
        if *var == v && (*op == "<" && va < *value || *op == ">" && va > *value) {
            return Some(target);
        }
    }
    None
}

fn one(input: &str) -> u32 {
    let mut splits = input.split("\n\n");
    let rules = splits.next().unwrap();
    let ratings = splits.next().unwrap();

    let mut workflows: HashMap<&str, Vec<Rule>> = HashMap::default();

    for line in rules.lines() {
        let splits = line.split_once('{').unwrap();
        let name = splits.0;
        let mut rules = vec![];
        for rule in splits.1[..splits.1.len() - 1].split(',') {
            if let Some((condition, target)) = rule.split_once(':') {
                let var = &condition[0..1];
                let op = &condition[1..2];
                let value = &condition[2..];
                rules.push(Rule::Cmp {
                    var,
                    op,
                    target,
                    value: value.parse().unwrap(),
                });
            } else {
                rules.push(Rule::One(rule));
            }
        }
        workflows.insert(name, rules);
    }
    let mut s = 0;
    for rating in ratings.lines() {
        let rating: HashMap<&str, u32> = rating[1..rating.len() - 1]
            .split(',')
            .map(|i| {
                let (var, value) = i.split_once('=').unwrap();
                (var, value.parse::<u32>().unwrap())
            })
            .collect();
        s += eval_rules(&workflows, &rating, "in");
    }
    s
}

fn count<'a>(
    workflows: &'a HashMap<&str, Vec<Rule>>,
    rating: &mut HashMap<&'a str, (u32, u32)>,
    name: &str,
) -> u64 {
    if name == "R" {
        return 0;
    }
    if name == "A" {
        let mut s = 1;
        for (low, high) in rating.values() {
            s *= (high - low) as u64 + 1;
        }
        return s;
    }
    let mut total = 0;

    let rules = workflows.get(name).unwrap();
    for rule in rules {
        if let Rule::Cmp {
            var,
            op,
            value,
            target,
        } = rule
        {
            let (low, high) = *rating.get(var).unwrap();
            let true_range;
            let false_range;
            if *op == "<" {
                true_range = (low, min(*value - 1, high));
                false_range = (max(*value, low), high);
            } else {
                true_range = (max(*value + 1, low), high);
                false_range = (low, min(*value, high));
            }
            if true_range.0 <= true_range.1 {
                let mut rr = rating.clone();
                rr.insert(var, true_range);
                total += count(workflows, &mut rr, target)
            }
            if false_range.0 <= false_range.1 {
                rating.insert(var, false_range);
            }
        }
    }
    if let Rule::One(ch) = rules.last().unwrap() {
        total += count(workflows, rating, ch);
    }
    total
}

fn two(input: &str) -> u64 {
    let mut splits = input.split("\n\n");
    let rules = splits.next().unwrap();

    let mut workflows: HashMap<&str, Vec<Rule>> = HashMap::default();

    for line in rules.lines() {
        let splits = line.split_once('{').unwrap();
        let name = splits.0;
        let mut rules = vec![];
        for rule in splits.1[..splits.1.len() - 1].split(',') {
            if let Some((condition, target)) = rule.split_once(':') {
                let var = &condition[0..1];
                let op = &condition[1..2];
                let value = &condition[2..];
                rules.push(Rule::Cmp {
                    var,
                    op,
                    target,
                    value: value.parse().unwrap(),
                });
            } else {
                rules.push(Rule::One(rule));
            }
        }
        workflows.insert(name, rules);
    }

    count(
        &workflows,
        &mut HashMap::from([
            ("x", (1, 4000)),
            ("a", (1, 4000)),
            ("s", (1, 4000)),
            ("m", (1, 4000)),
        ]),
        "in",
    )
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";
        assert_eq!(19114, one(input.trim()));
        assert_eq!(167409079868000, two(input.trim()));
    }

    #[test]
    fn test_answer() {
        println!("{}", one(&read_to_string("data/2023/day19.txt").unwrap()));
        println!("{}", two(&read_to_string("data/2023/day19.txt").unwrap()));
    }
}
