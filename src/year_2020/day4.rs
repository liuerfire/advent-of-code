use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl Passport {
    fn new() -> Self {
        Self {
            byr: String::new(),
            iyr: String::new(),
            eyr: String::new(),
            hgt: String::new(),
            hcl: String::new(),
            ecl: String::new(),
            pid: String::new(),
            cid: String::new(),
        }
    }

    fn update(&mut self, m: &HashMap<String, String>) {
        self.byr = m.get("byr").cloned().unwrap_or(String::new());
        self.iyr = m.get("iyr").cloned().unwrap_or(String::new());
        self.eyr = m.get("eyr").cloned().unwrap_or(String::new());
        self.hgt = m.get("hgt").cloned().unwrap_or(String::new());
        self.hcl = m.get("hcl").cloned().unwrap_or(String::new());
        self.ecl = m.get("ecl").cloned().unwrap_or(String::new());
        self.pid = m.get("pid").cloned().unwrap_or(String::new());
        self.cid = m.get("cid").cloned().unwrap_or(String::new());
    }

    fn is_valid(&self) -> bool {
        match self.byr.parse::<i32>() {
            Ok(v) => {
                if v < 1920 || v > 2002 {
                    return false;
                }
            }
            _ => return false,
        }
        match self.iyr.parse::<i32>() {
            Ok(v) => {
                if v < 2010 || v > 2020 {
                    return false;
                }
            }
            _ => return false,
        }
        match self.eyr.parse::<i32>() {
            Ok(v) => {
                if v < 2020 || v > 2030 {
                    return false;
                }
            }
            _ => return false,
        }

        if self.hgt.len() <= 2 {
            return false;
        }
        let unit = &self.hgt[self.hgt.len() - 2..];
        let hgt = &self.hgt[0..self.hgt.len() - 2].parse::<i32>();
        let value = match hgt {
            Ok(v) => v,
            _ => return false,
        };
        match unit {
            "cm" => {
                if value < &150 || value > &193 {
                    return false;
                }
            }
            "in" => {
                if value < &59 || value > &76 {
                    return false;
                }
            }
            _ => return false,
        }

        if self.hcl.len() != 7 {
            return false;
        }
        for (i, s) in self.hcl.chars().enumerate() {
            let s = s.to_string();
            if i == 0 && s != "#" {
                return false;
            }
            if i >= 1 {
                if !f1(&s) && !f2(&s) {
                    return false;
                }
            }
        }

        match self.ecl.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
            _ => return false,
        }

        if self.pid.len() != 9 {
            return false;
        }
        for s in self.pid.chars() {
            if !f2(&s.to_string()) {
                return false;
            }
        }
        true
    }
}

fn f1(c: &str) -> bool {
    let x = vec!["a", "b", "c", "d", "e", "f"];
    return x.contains(&c);
}

fn f2(c: &str) -> bool {
    let x = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    return x.contains(&c);
}

pub fn passports() {
    let content = fs::read_to_string("./src/year_2020/day4-input.txt").expect("shit");

    let mut count = 0;
    let mut p = Passport::new();

    let mut map = HashMap::new();
    let lines: Vec<&str> = content.trim().split("\n\n").collect();
    for line in lines {
        let fields: Vec<&str> = line.split_whitespace().collect();
        for field in fields {
            let f: Vec<&str> = field.split(":").collect();
            map.insert(f[0].to_string(), f[1].to_string());
        }
        p.update(&map);
        if p.is_valid() {
            count += 1;
        }
        map.clear();
    }

    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passports() {
        passports();
    }
}
