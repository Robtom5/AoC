// use std::collections::HashSet;
use std::fs;

fn find_duplicates(content: &str) -> u32 {
    let mut priority_sum = 0;
    let lines = content.lines();

    for line in lines {
        let len = line.chars().count() / 2;

        let left = line[0..len].to_bit();
        let right = line[len..].to_bit();

        let diff: u64 = left & right;

        priority_sum += match diff & diff - 1 {
            0 => diff.ilog2(),
            _ => panic!(""),
        }
    }
    return priority_sum;
}

fn find_badge(content: &str) -> u32 {
    let mut badges_sum = 0;
    let lines: Vec<&str> = content.lines().collect();
    for i in (0..lines.len()).step_by(3) {
        let diff: u64 = lines[i].to_bit() & lines[i + 1].to_bit() & lines[i + 2].to_bit();
        badges_sum += match diff & diff - 1 {
            0 => diff.ilog2(),
            _ => panic!(""),
        }
    }
    badges_sum
}

trait BitEncode {
    fn to_bit(&self) -> u64;
}

impl BitEncode for char {
    fn to_bit(&self) -> u64 {
        return match self.is_ascii_uppercase() {
            true => self.to_digit(36).unwrap() + 17,
            false => self.to_digit(36).unwrap() - 9,
        } as u64;
    }
}

impl BitEncode for str {
    fn to_bit(&self) -> u64 {
        let mut bit: u64 = 0b0;
        for c in self.chars() {
            bit |= 1 << c.to_bit();
        }
        return bit;
    }
}

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }
    .to_string();

    let contents = fs::read_to_string(fp).expect("Should be able to read file");

    let total_score = find_duplicates(&contents);
    let badge_score = find_badge(&contents);

    println!("Total score {total_score}");
    println!("Badge score {badge_score}");
}
