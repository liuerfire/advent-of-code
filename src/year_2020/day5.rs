use std::fs;

pub fn seat_id() {
    let content = fs::read_to_string("./src/year_2020/day5-input.txt").expect("shit");
    let lines: Vec<&str> = content.trim().split("\n").collect();

    let mut seats = Vec::new();
    for line in lines {
        let row = get_row(&line[0..7]);
        let column = get_column(&line[7..]);
        seats.push(row * 8 + column);
    }

    seats.sort();
    println!("{}", seats[seats.len() - 1]);

    for i in 0..seats.len() - 2 {
        if seats[i] + 1 != seats[i + 1] {
            println!("{}", seats[i] + 1);
        }
    }
}
fn get_row(s: &str) -> i32 {
    let mut min = 0;
    let mut max = 127;
    for (i, c) in s.chars().enumerate() {
        let c = c.to_string();
        match c.as_str() {
            "F" => {
                max = lower_half(min, max);
                if i == s.len() - 1 {
                    return max;
                }
            }
            "B" => {
                min = upper_half(min, max);
                if i == s.len() - 1 {
                    return min;
                }
            }
            _ => {}
        }
    }
    0
}

fn get_column(s: &str) -> i32 {
    let mut min = 0;
    let mut max = 7;
    for (i, c) in s.chars().enumerate() {
        let c = c.to_string();
        match c.as_str() {
            "L" => {
                max = lower_half(min, max);
                if i == s.len() - 1 {
                    return max;
                }
            }
            "R" => {
                min = upper_half(min, max);
                if i == s.len() - 1 {
                    return min;
                }
            }
            _ => {}
        }
    }
    0
}

fn lower_half(x: i32, y: i32) -> i32 {
    x - 1 + (y - x + 1) / 2
}

fn upper_half(x: i32, y: i32) -> i32 {
    y + 1 - (y - x + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5() {
        assert_eq!(63, lower_half(0, 127));
        assert_eq!(32, upper_half(0, 63));
        assert_eq!(47, lower_half(32, 63));
        assert_eq!(44, upper_half(40, 47));
        assert_eq!(44, lower_half(44, 45));
        assert_eq!(44, get_row("FBFBBFF"));
        assert_eq!(5, get_column("RLR"));

        seat_id();
    }
}
