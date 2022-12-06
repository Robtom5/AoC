use regex::Regex;
use std::fs;

fn overlaps(contents: &str) -> (String, String) {
    let re = Regex::new(r"^(?P<s1>\d+)-(?P<e1>\d+),(?P<s2>\d+)-(?P<e2>\d+)$").unwrap();

    let mut wasted_elves = 0;
    let mut inefficient_elves = 0;
    for l in contents.lines() {
        let elf_pos = match re.is_match(l) {
            true => split_regex(re.captures_iter(l)),
            false => continue,
        };
        wasted_elves += match elf_pos {
            (a, b, x, y) if (a <= x) && (b >= y) => 1,
            (a, b, x, y) if (a >= x) && (b <= y) => 1,
            _ => 0,
        };
        inefficient_elves += match elf_pos {
            (a, b, x, _) if (b >= x) && (a <= x) => 1,
            (a, _, x, y) if (y >= a) && (x <= a) => 1,
            _ => 0,
        };
    }
    return (wasted_elves.to_string(), inefficient_elves.to_string());
}

fn split_regex(mut captures: regex::CaptureMatches) -> (usize, usize, usize, usize) {
    let cap = captures.next().unwrap();
    return (
        cap["s1"].parse::<usize>().unwrap(),
        cap["e1"].parse::<usize>().unwrap(),
        cap["s2"].parse::<usize>().unwrap(),
        cap["e2"].parse::<usize>().unwrap(),
    );
}

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }
    .to_string();

    let contents = fs::read_to_string(fp).expect("Should be able to read file");

    let (part1, part2) = overlaps(&contents);
    println!("Part 1: {part1} Part 2: {part2}");
}
