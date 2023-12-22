use indexmap::IndexMap;

fn one(input: &str) -> usize {
    let mut s = 0;
    for elem in input.split(',') {
        s += hash(elem);
    }
    s
}

fn hash(cs: &str) -> usize {
    let mut current = 0;
    for c in cs.chars() {
        current += c as usize;
        current *= 17;
        current %= 256;
    }
    current
}

fn two(input: &str) -> usize {
    let mut boxes: Vec<IndexMap<String, usize>> = vec![];
    for _ in 0..256 {
        boxes.push(IndexMap::default());
    }
    for elem in input.split(',') {
        if elem.ends_with('-') {
            let label = &elem[0..elem.len() - 1];
            let box_id = hash(label);
            let x = &mut boxes[box_id];
            x.shift_remove(label).unwrap_or_default();
        } else {
            let (label, n) = elem.split_once('=').unwrap();
            let n: usize = n.parse().unwrap();
            boxes[hash(label)].insert(label.to_string(), n);
        }
    }
    let mut s = 0;
    let mut i = 0;
    while i < boxes.len() {
        let mut j = 0;
        let values: Vec<usize> = boxes[i].values().copied().collect();
        while j < values.len() {
            s += (i + 1) * (j + 1) * values[j];
            j += 1;
        }
        i += 1;
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
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";
        assert_eq!(1320, one(input.trim()));
        assert_eq!(145, two(input.trim()));
    }

    #[test]
    fn test_answer() {
        println!(
            "{}",
            one(&read_to_string("data/2023/day15.txt").unwrap().trim())
        );
        println!(
            "{}",
            two(&read_to_string("data/2023/day15.txt").unwrap().trim())
        );
    }
}
