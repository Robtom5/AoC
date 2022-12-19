use std::fs;

fn part1(contents: &str) -> String {
    "".to_string()
}

fn part2(contents: &str) -> String {
    "".to_string()
}

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }
    .to_string();

    let contents = fs::read_to_string(fp).expect("Should be able to read file");

    let res1 = part1(&contents);
    let res2 = part2(&contents);

    println!("Part 1 {res1} Part 2 {res2}")
}
