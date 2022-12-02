use core::cmp::Ordering;
use core::str;
use std::fs;
use regex::Regex;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Hand{
    Rock=1, // Rock
    Paper=2, // Paper
    Scissors=3, // Scissors
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self)-> Ordering {
        let diff = (*self as u8 - *other as u8).rem_euclid(2);
        println!("{diff}");
        match diff {
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            2 => Ordering::Less,
            _ => panic!("Maths is broken")
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let diff = (*self as u8 - *other as u8).rem_euclid(2);
        println!("{diff}");
        match diff {
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            2 => Ordering::Less,
            _ => panic!("Maths is broken")
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Result{
    Loss=0,
    Draw=3,
    Win=6,
}

fn play_round((opp, you): (Hand, Hand)) -> u8{
    let result = match &you{
        _ if opp > you => Result::Loss,
        _ if opp == you => Result::Draw,
        _ if opp < you => Result::Win,
        _ => panic!("Unknown result")
    };

    let score = result as u8 + you as u8;
    #[cfg(debug_assertions)]
    println!("Opp {opp:?} You {you:?} Result {result:?} Score {score}");
    return score;
}

fn parse_hand(line: &str) -> (Hand, Hand){   
    let re = Regex::new(r"(?P<opp>[A-C]) (?P<you>[X-Z])");

    let groups = re.unwrap().captures(line).unwrap();

    let opp: Hand = parse_opp(groups.name("opp").unwrap().as_str());
    let you: Hand = parse_you_1(groups.name("you").unwrap().as_str());

    let _t = opp > you;
    println!("{_t}");

    return (opp, you);
}

fn parse_opp(shape: &str) -> Hand{
    return match shape {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissors,
        _ => panic!("Invalid shape '{shape}'"),
    }
}

fn parse_you_1(shape: &str) -> Hand{
    return match shape {
        "X" => Hand::Rock,
        "Y" => Hand::Paper,
        "Z" => Hand::Scissors,
        _ => panic!("Invalid shape '{shape}'"),
    }
}

fn all_scores(content: &str)-> Vec<i32>{
    let lines = content.lines();
    let mut scores: Vec<i32> = Vec::new();
  
    for l in lines{
        let score = play_round(parse_hand(l)) as i32;
        let validation = brute_line(l);

        assert_eq!(score, validation, "Bad line {} => {} {}", l, score, validation);
        scores.push(score);
    }

    return scores
}

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

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }.to_string();

    let contents = fs::read_to_string(fp)
        .expect("Should be able to read file");


    let scores = all_scores(&contents);

    let mut score = 0;
    for s in scores{
        // #[cfg(debug_assertions)]
        // println!("Score {s}");
        score += s;
    }
    println!("Final Score {score}");

    let dumb_scores = brute_lines(&contents);
    let dumb_total: i32 = dumb_scores.iter().sum();
    println!("Dumb Score {dumb_total}");

    // 11552 too high
}
