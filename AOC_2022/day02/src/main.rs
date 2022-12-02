use core::str;
use std::fs;



fn brute_lines(content: &str)-> Vec<i32>{
    let lines = content.lines();
    let mut scores: Vec<i32> = Vec::new();

    for l in lines {
        let score: i32 = brute_line(l);
        scores.push(score);
    }
    return scores;
}

fn brute_line(line: &str) -> i32 {
    match line{
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => 0,
    }
}

fn brute_lines2(content: &str)-> Vec<i32>{
    let lines = content.lines();
    let mut scores: Vec<i32> = Vec::new();

    for l in lines {
        let score: i32 = brute_line2(l);
        scores.push(score);
    }
    return scores;
}


fn brute_line2(line: &str) -> i32 {
    match line{
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => 0,
    }
}

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }.to_string();

    let contents = fs::read_to_string(fp)
        .expect("Should be able to read file");


    let dumb_scores = brute_lines(&contents);
    let dumb_total: i32 = dumb_scores.iter().sum();
    println!("Dumb Score {dumb_total}");


    let dumb_scores2 = brute_lines2(&contents);
    let dumb_total2: i32 = dumb_scores2.iter().sum();
    println!("Dumb Score 2 {dumb_total2}");

    // 11552 too high
}
